use std::mem::discriminant;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OS {
    Windows,
    Linux,
    Macos
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Arch {
    X86_64,
    X86
}

impl PartialEq for OS {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl PartialEq for Arch {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

pub(crate) struct Device {
    pub(crate) os: OS,
    pub(crate) arch: Arch
}

impl Device {
    #[cfg(target_os = "windows")]
    #[cfg(target_arch = "x86_64")]
    pub(crate) fn get_specific() -> Device {
        Device {
            os: OS::Windows,
            arch: Arch::X86_64
        }
    }

    #[cfg(target_os = "linux")]
    #[cfg(target_arch = "x86_64")]
    pub(crate) fn get_specific() -> Device {
        Device {
            os: OS::Linux,
            arch: Arch::X86_64
        }
    }

    #[cfg(target_os = "macos")]
    #[cfg(target_arch = "x86_64")]
    pub(crate) fn get_specific() -> Device {
        Device {
            os: OS::Macos,
            arch: Arch::X86_64
        }
    }
}
