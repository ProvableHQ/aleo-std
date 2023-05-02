// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the aleo-std library.

// The aleo-std library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The aleo-std library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the aleo-std library. If not, see <https://www.gnu.org/licenses/>.

/// Uses Rust's `cpuid` function from the `arch` module.
pub(crate) mod native_cpuid {
    /// Low-level data-structure to store result of cpuid instruction.
    #[derive(Copy, Clone, Eq, PartialEq)]
    #[repr(C)]
    pub struct CpuIdResult {
        /// Return value EAX register
        pub eax: u32,
        /// Return value EBX register
        pub ebx: u32,
        /// Return value ECX register
        pub ecx: u32,
        /// Return value EDX register
        pub edx: u32,
    }

    #[allow(unreachable_code)]
    #[allow(unused_variables)]
    pub fn cpuid_count(a: u32, c: u32) -> CpuIdResult {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(target_env = "sgx")))]
        {
            #[cfg(all(target_arch = "x86", target_feature = "sse"))]
            use core::arch::x86 as arch;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64 as arch;

            // Safety: CPUID is supported on all x86_64 CPUs and all x86 CPUs with SSE, but not by SGX.
            let result = unsafe { arch::__cpuid_count(a, c) };
            return CpuIdResult {
                eax: result.eax,
                ebx: result.ebx,
                ecx: result.ecx,
                edx: result.edx,
            };
        }

        CpuIdResult {
            eax: 22,
            ebx: 1970169159,
            ecx: 1818588270,
            edx: 1231384169,
        }
    }
}

///
/// Vendor Info String (LEAF=0x0)
///
/// The vendor info is a 12-byte (96 bit) long string stored in `ebx`, `edx` and
/// `ecx` by the corresponding `cpuid` instruction.
///
#[derive(PartialEq, Eq)]
#[repr(C)]
struct VendorInfo {
    ebx: u32,
    edx: u32,
    ecx: u32,
}

impl VendorInfo {
    /// Return vendor identification as string, such as "AuthenticAMD" or "GenuineIntel".
    fn as_str(&self) -> &str {
        let brand_string_start = self as *const VendorInfo as *const u8;
        let slice = unsafe {
            // Safety: VendorInfo is laid out with repr(C) and exactly
            // 12 byte long without any padding.
            core::slice::from_raw_parts(brand_string_start, core::mem::size_of::<VendorInfo>())
        };
        core::str::from_utf8(slice).unwrap_or("InvalidVendorString")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cpu {
    AMD,
    Intel,
    Unknown,
}

///
/// Returns a new Cpu enum.
///
/// The vendor leaf will contain a ASCII readable string such as "GenuineIntel"
/// for Intel CPUs or "AuthenticAMD" for AMD CPUs.
///
#[allow(clippy::absurd_extreme_comparisons)]
pub fn get_cpu() -> Cpu {
    const EAX_VENDOR_INFO: u32 = 0x0;

    // Check if a non extended leaf  (`val`) is supported.
    let vendor_leaf = native_cpuid::cpuid_count(EAX_VENDOR_INFO, 0);
    let is_leaf_supported = EAX_VENDOR_INFO <= vendor_leaf.eax;

    match is_leaf_supported {
        true => {
            let vendor = VendorInfo {
                ebx: vendor_leaf.ebx,
                ecx: vendor_leaf.ecx,
                edx: vendor_leaf.edx,
            };

            match vendor.as_str() {
                "AuthenticAMD" => Cpu::AMD,
                "GenuineIntel" => Cpu::Intel,
                _ => Cpu::Unknown,
            }
        }
        false => Cpu::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu() {
        println!("{:?}", get_cpu());
    }
}
