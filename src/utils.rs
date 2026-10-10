// Take a look at the license at the top of the repository in the LICENSE file.

cfg_select! {
    all(
        feature = "multithread",
        not(feature = "unknown-ci"),
        not(all(target_os = "macos", feature = "apple-sandbox")),
    ) => {
        /// Converts the value into a parallel iterator if the `multithread` feature is enabled.
        /// Uses the `rayon::iter::IntoParallelIterator` trait.
        #[cfg(all(
            feature = "multithread",
            not(feature = "unknown-ci"),
            not(all(target_os = "macos", feature = "apple-sandbox")),
        ))]
        #[allow(dead_code)]
        pub(crate) fn into_iter<T>(val: T) -> T::Iter
        where
            T: rayon::iter::IntoParallelIterator,
        {
            val.into_par_iter()
        }

        // /// Converts the value into a parallel mutable iterator if the `multithread` feature is
        // /// enabled. Uses the `rayon::iter::IntoParallelRefMutIterator` trait.
        // #[cfg(feature = "component")]
        // pub(crate) fn into_iter_mut<'a, T>(
        //     val: &'a mut T,
        // ) -> <T as rayon::iter::IntoParallelRefMutIterator<'a>>::Iter
        // where
        //     T: rayon::iter::IntoParallelRefMutIterator<'a> + ?Sized,
        // {
        //     val.par_iter_mut()
        // }
    }
    _ => {
        /// Converts the value into a sequential iterator if the `multithread` feature is disabled.
        /// Uses the `std::iter::IntoIterator` trait.
        #[allow(dead_code)]
        pub(crate) fn into_iter<T>(val: T) -> T::IntoIter
        where
            T: IntoIterator,
        {
            val.into_iter()
        }

        // In the multithreaded version of `into_iter_mut` above, the `&mut` on the argument is
        // indicating the parallel iterator is an exclusive reference. In the non-multithreaded
        // case, the `&mut` is already part of `T` and specifying it will result in the argument
        // being `&mut &mut T`.

        // /// Converts the value into a sequential mutable iterator if the `multithread` feature is
        // /// disabled. Uses the `std::iter::IntoIterator` trait.
        // #[cfg(feature = "component")]
        // pub(crate) fn into_iter_mut<T>(val: T) -> T::IntoIter
        // where
        //     T: IntoIterator,
        // {
        //     val.into_iter()
        // }
    }
}

/// The CPU's vendor id (`cpuid` leaf `0`) and brand string (leaves `0x8000_0002..=0x8000_0004`).
///
/// The brand is empty on a CPU without the extended leaves, and the vendor id is `None` when its
/// bytes are not UTF-8.
#[cfg(all(
    feature = "system",
    not(feature = "unknown-ci"),
    any(windows, target_os = "toyos"),
    any(target_arch = "x86", target_arch = "x86_64"),
))]
pub(crate) fn cpuid_vendor_id_and_brand() -> (Option<String>, String) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::__cpuid;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::__cpuid;

    let brand = if __cpuid(0x8000_0000).eax >= 0x8000_0004 {
        let registers: Vec<u32> = (0x8000_0002..=0x8000_0004)
            .flat_map(|leaf| {
                let r = __cpuid(leaf);
                [r.eax, r.ebx, r.ecx, r.edx]
            })
            .collect();
        cpuid_text(&registers).unwrap_or_default()
    } else {
        String::new()
    };
    let r = __cpuid(0);
    (cpuid_text(&[r.ebx, r.edx, r.ecx]), brand)
}

/// The text `registers` spell, each register's bytes in little-endian order, up to the first NUL;
/// `None` if that is not UTF-8.
#[cfg(any(
    test,
    all(
        feature = "system",
        not(feature = "unknown-ci"),
        any(windows, target_os = "toyos"),
        any(target_arch = "x86", target_arch = "x86_64"),
    )
))]
fn cpuid_text(registers: &[u32]) -> Option<String> {
    let bytes = registers
        .iter()
        .flat_map(|r| r.to_le_bytes())
        .take_while(|&b| b != 0)
        .collect();
    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::cpuid_text;

    #[test]
    fn cpuid_text_reads_registers_in_order_up_to_nul() {
        // "GenuineIntel" as leaf 0 returns it: ebx, edx, ecx.
        assert_eq!(
            cpuid_text(&[0x756e_6547, 0x4965_6e69, 0x6c65_746e]).as_deref(),
            Some("GenuineIntel")
        );
        // A brand string padded with NULs stops at the first one.
        assert_eq!(
            cpuid_text(&[0x2055_5043, 0x0000_3121, 0x6464_6464]).as_deref(),
            Some("CPU !1")
        );
        assert_eq!(cpuid_text(&[0]).as_deref(), Some(""));
        assert_eq!(cpuid_text(&[0x0000_00ff]), None);
    }
}
