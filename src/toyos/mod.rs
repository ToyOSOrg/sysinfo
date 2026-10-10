// Take a look at the license at the top of the repository in the LICENSE file.

// ToyOS answers the machine-wide facts every process may read: memory, the CPUs, uptime and the
// build in `/system/etc/os-release`. Processes, disks, components, networks and users are behind
// capabilities a library cannot take on its caller's behalf, so they are the `unknown` backend's.

cfg_select! {
    feature = "system" => {
        pub mod cpu;
        #[path = "../unknown/motherboard.rs"]
        pub mod motherboard;
        #[path = "../unknown/process.rs"]
        pub mod process;
        #[path = "../unknown/product.rs"]
        pub mod product;
        pub mod system;

        pub(crate) use self::cpu::CpuInner;
        pub(crate) use self::motherboard::MotherboardInner;
        pub(crate) use self::process::ProcessInner;
        pub(crate) use self::product::ProductInner;
        pub(crate) use self::system::SystemInner;
        pub use self::system::{MINIMUM_CPU_UPDATE_INTERVAL, SUPPORTED_SIGNALS};
    }
    _ => {}
}
cfg_select! {
    feature = "disk" => {
        #[path = "../unknown/disk.rs"]
        pub mod disk;

        pub(crate) use self::disk::{DiskInner, DisksInner};
    }
    _ => {}
}
cfg_select! {
    feature = "component" => {
        #[path = "../unknown/component.rs"]
        pub mod component;

        pub(crate) use self::component::{ComponentInner, ComponentsInner};
    }
    _ => {}
}
cfg_select! {
    feature = "network" => {
        #[path = "../unknown/network.rs"]
        pub mod network;

        pub(crate) use self::network::{NetworkDataInner, NetworksInner};
    }
    _ => {}
}
cfg_select! {
    feature = "user" => {
        #[path = "../unknown/groups.rs"]
        pub mod groups;
        #[path = "../unknown/users.rs"]
        pub mod users;

        pub(crate) use self::groups::get_groups;
        pub(crate) use self::users::{get_users, UserInner};
    }
    _ => {}
}

#[doc = include_str!("../../md_doc/is_supported.md")]
pub const IS_SUPPORTED_SYSTEM: bool = false;

// Make formattable by rustfmt.
#[cfg(any())]
mod cpu;
#[cfg(any())]
mod system;
