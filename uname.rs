use std::ffi::CStr;
use std::io;

use libc::{uname, utsname};

use super::Error;

/// Result of `uname`, fields can be accessed through methods.
pub struct Info {
    utsname: utsname,
}

macro_rules! info_methods {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis fn $name:ident;
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis fn $name(&self) -> Result<&str, Error> {
                let bytes = &self.utsname.$name;
                // Make sure the string is null-terminated so we don't overflow.
                if !bytes.iter().any(|b| *b == 0) {
                    return Err(Error::Unknown);
                }
                unsafe { CStr::from_ptr(bytes.as_ptr()) }
                    .to_str()
                    .map_err(|_| Error::Unknown)
            }
        )*
    };
}

impl Info {
    pub(crate) fn new() -> Result<Info, Error> {
        let mut info = Info {
            utsname: unsafe { std::mem::zeroed() },
        };
        let ret = unsafe { uname(&mut info.utsname as *mut utsname) };
        if ret != 0 {
            return Err(io::Error::last_os_error().into());
        }
        Ok(info)
    }

    info_methods!(
        /// Kernel Name, for example "Linux".
        pub fn sysname;
        /// Network node hostname, this is usually the same as the hostname.
        pub fn nodename;
        /// Kernel release, for example "5.10.4-arch2-1".
        pub fn release;
        /// Kernel version, for example "#1 SMP PREEMPT Fri, 01 Jan 2021 05:29:53 +0000".
        pub fn version;
        /// Machine hardware name, for example "x86_64".
        ///
        /// Note that this isn't yet supported on non-unix systems.
        pub fn machine;
    );

    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "fuchsia",
        target_os = "redox"
    ))]
    info_methods!(
        /// Domain name of the system.
        ///
        /// This is only supported on linux, android, fuchsia and redox.
        pub fn domainname;
    );
}
