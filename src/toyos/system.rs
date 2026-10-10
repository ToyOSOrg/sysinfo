// Take a look at the license at the top of the repository in the LICENSE file.

use crate::toyos::cpu::{CpuInner, vendor_id_and_brand};
use crate::utils::os_release_value;
use crate::{
    Cpu, CpuRefreshKind, LoadAvg, MemoryRefreshKind, Pid, Process, ProcessRefreshKind,
    ProcessesToUpdate,
};

use std::collections::HashMap;
use std::time::Duration;

use ::toyos::system::SYSINFO_HEADER_SIZE;
use toyos_abi::syscall::SysinfoHeader;

declare_signals! {
    (),
    _ => None,
}

#[doc = include_str!("../../md_doc/supported_signals.md")]
pub const SUPPORTED_SIGNALS: &[crate::Signal] = supported_signals();
#[doc = include_str!("../../md_doc/minimum_cpu_update_interval.md")]
pub const MINIMUM_CPU_UPDATE_INTERVAL: Duration = Duration::from_millis(0);

/// Which build the running system is, in the os-release(5) format.
const OS_RELEASE: &str = "/system/etc/os-release";

/// The machine-wide facts every process may read; `None` if the kernel refused them.
fn header() -> Option<SysinfoHeader> {
    let mut buf = [0; SYSINFO_HEADER_SIZE];
    (::toyos::system::sysinfo(&mut buf) == SYSINFO_HEADER_SIZE).then(|| SysinfoHeader::decode(&buf))
}

fn os_release(key: &str) -> Option<String> {
    os_release_value(&std::fs::read_to_string(OS_RELEASE).ok()?, key)
}

pub(crate) struct SystemInner {
    process_list: HashMap<Pid, Process>,
    mem_total: u64,
    mem_used: u64,
    cpus: Vec<Cpu>,
}

impl SystemInner {
    pub(crate) fn new() -> Self {
        Self {
            process_list: Default::default(),
            mem_total: 0,
            mem_used: 0,
            cpus: Vec::new(),
        }
    }

    pub(crate) fn refresh_memory_specifics(&mut self, refresh_kind: MemoryRefreshKind) {
        if !refresh_kind.ram() {
            return;
        }
        if let Some(header) = header() {
            self.mem_total = header.memory_total;
            self.mem_used = header.memory_used;
        }
    }

    pub(crate) fn cgroup_limits(&self) -> Option<crate::CGroupLimits> {
        None
    }

    pub(crate) fn refresh_cpu_specifics(&mut self, refresh_kind: CpuRefreshKind) {
        if self.cpus.is_empty() {
            self.refresh_cpu_list(refresh_kind);
        }
    }

    pub(crate) fn refresh_cpu_list(&mut self, _refresh_kind: CpuRefreshKind) {
        let Some(header) = header() else {
            self.cpus.clear();
            return;
        };
        let (vendor_id, brand) = vendor_id_and_brand();
        self.cpus = (0..header.cpus)
            .map(|nb| Cpu {
                inner: CpuInner::new(format!("cpu{nb}"), vendor_id.clone(), brand.clone()),
            })
            .collect();
    }

    pub(crate) fn refresh_processes_specifics(
        &mut self,
        _processes_to_update: ProcessesToUpdate<'_>,
        _refresh_kind: ProcessRefreshKind,
    ) -> usize {
        0
    }

    // COMMON PART
    //
    // Need to be moved into a "common" file to avoid duplication.

    pub(crate) fn processes(&self) -> &HashMap<Pid, Process> {
        &self.process_list
    }

    pub(crate) fn processes_mut(&mut self) -> &mut HashMap<Pid, Process> {
        &mut self.process_list
    }

    pub(crate) fn process(&self, _pid: Pid) -> Option<&Process> {
        None
    }

    pub(crate) fn global_cpu_usage(&self) -> f32 {
        0.
    }

    pub(crate) fn cpus(&self) -> &[Cpu] {
        &self.cpus
    }

    pub(crate) fn total_memory(&self) -> u64 {
        self.mem_total
    }

    pub(crate) fn free_memory(&self) -> u64 {
        self.mem_total.saturating_sub(self.mem_used)
    }

    pub(crate) fn available_memory(&self) -> u64 {
        self.free_memory()
    }

    pub(crate) fn used_memory(&self) -> u64 {
        self.mem_used
    }

    pub(crate) fn total_swap(&self) -> u64 {
        0
    }

    pub(crate) fn free_swap(&self) -> u64 {
        0
    }

    pub(crate) fn used_swap(&self) -> u64 {
        0
    }

    pub(crate) fn uptime() -> u64 {
        header().map_or(0, |header| header.uptime_ns / 1_000_000_000)
    }

    pub(crate) fn boot_time() -> u64 {
        ::toyos::system::clock_epoch().map_or(0, |now| now.saturating_sub(Self::uptime()))
    }

    pub(crate) fn load_average() -> LoadAvg {
        LoadAvg {
            one: 0.,
            five: 0.,
            fifteen: 0.,
        }
    }

    pub(crate) fn name() -> Option<String> {
        os_release("NAME")
    }

    pub(crate) fn long_os_version() -> Option<String> {
        os_release("PRETTY_NAME")
    }

    // The kernel is part of the build os-release names.
    pub(crate) fn kernel_version() -> Option<String> {
        os_release("VERSION_ID")
    }

    pub(crate) fn os_version() -> Option<String> {
        os_release("VERSION_ID")
    }

    pub(crate) fn distribution_id() -> String {
        os_release("ID").unwrap_or_else(|| std::env::consts::OS.to_owned())
    }

    pub(crate) fn distribution_id_like() -> Vec<String> {
        os_release("ID_LIKE")
            .map(|like| like.split_ascii_whitespace().map(String::from).collect())
            .unwrap_or_default()
    }

    pub(crate) fn kernel_name() -> Option<&'static str> {
        Some("ToyOS")
    }

    pub(crate) fn host_name() -> Option<String> {
        None
    }

    pub(crate) fn cpu_arch() -> Option<String> {
        None
    }

    pub(crate) fn physical_core_count() -> Option<usize> {
        None
    }

    pub(crate) fn open_files_limit() -> Option<usize> {
        None
    }
}
