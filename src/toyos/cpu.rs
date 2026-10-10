// Take a look at the license at the top of the repository in the LICENSE file.

pub(crate) struct CpuInner {
    name: String,
    vendor_id: String,
    brand: String,
}

impl CpuInner {
    pub(crate) fn new(name: String, vendor_id: String, brand: String) -> Self {
        Self {
            name,
            vendor_id,
            brand,
        }
    }

    pub(crate) fn cpu_usage(&self) -> f32 {
        0.0
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn frequency(&self) -> u64 {
        0
    }

    pub(crate) fn vendor_id(&self) -> &str {
        &self.vendor_id
    }

    pub(crate) fn brand(&self) -> &str {
        &self.brand
    }
}

/// The vendor id and brand string every CPU of the machine shares.
#[cfg(target_arch = "x86_64")]
pub(crate) fn vendor_id_and_brand() -> (String, String) {
    let (vendor_id, brand) = crate::utils::cpuid_vendor_id_and_brand();
    (vendor_id.unwrap_or_default(), brand)
}

/// ToyOS publishes no `MIDR_EL1`, and reading it from EL0 ends the process.
#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn vendor_id_and_brand() -> (String, String) {
    (String::new(), String::new())
}
