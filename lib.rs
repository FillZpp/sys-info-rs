//! #Introduction
//! This crate focuses on geting system information.
//!
//! For now it supports Linux, Mac OS X and Windows.
//! And now it can get information of kernel/cpu/memory/disk/load/hostname and so on.
//!

extern crate libc;

use std::ffi;
use std::fmt;
use std::io::{self, Read};
use std::fs::File;
use std::os::raw::c_char;

#[cfg(target_os = "macos")]
use libc::sysctl;
use libc::timeval;
use std::mem::size_of_val;
use std::ptr::null_mut;

use std::collections::HashMap;

static MAC_CTL_KERN: libc::c_int = 1;
static MAC_KERN_BOOTTIME: libc::c_int = 21;

/// System load average value.
#[repr(C)]
#[derive(Debug)]
pub struct LoadAvg {
    /// Average load within one minite.
    pub one: f64,
    /// Average load within five minites.
    pub five: f64,
    /// Average load within fifteen minites.
    pub fifteen: f64,
}

/// System memory information.
#[repr(C)]
#[derive(Debug)]
pub struct MemInfo {
    /// Total physical memory.
    pub total: u64,
    pub free: u64,
    pub avail: u64,

    pub buffers: u64,
    pub cached: u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free: u64,
}

/// Disk information.
#[repr(C)]
#[derive(Debug)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64,
}

/// Error types
#[derive(Debug)]
pub enum Error {
    UnsupportedSystem,
    ExecFailed(io::Error),
    IO(io::Error),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnsupportedSystem => write!(fmt, "System is not supported"),
            ExecFailed(ref e) => write!(fmt, "Execution failed: {}", e),
            IO(ref e) => write!(fmt, "IO error: {}", e),
            Unknown => write!(fmt, "An unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnsupportedSystem => "unsupported system",
            ExecFailed(_) => "execution failed",
            IO(_) => "io error",
            Unknown => "unknown error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        use self::Error::*;
        match *self {
            UnsupportedSystem => None,
            ExecFailed(ref e) => Some(e),
            IO(ref e) => Some(e),
            Unknown => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

extern "C" {
    fn get_os_type() -> *const i8;
    fn get_os_release() -> *const i8;

    fn get_cpu_num() -> u32;
    fn get_cpu_speed() -> u64;

    fn get_loadavg() -> LoadAvg;
    fn get_proc_total() -> u64;

    fn get_mem_info() -> MemInfo;
    fn get_disk_info() -> DiskInfo;
}


/// Get operation system type.
///
/// Such as "Linux", "Darwin", "Windows".
pub fn os_type() -> Result<String, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/sys/kernel/ostype")?.read_to_string(&mut s)?;
        s.pop(); // pop '\n'
        Ok(s)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        let typ = unsafe { ffi::CStr::from_ptr(get_os_type() as *const c_char).to_bytes() };
        Ok(String::from_utf8_lossy(typ).into_owned())
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get operation system release version.
///
/// Such as "3.19.0-gentoo"
pub fn os_release() -> Result<String, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/sys/kernel/osrelease")?.read_to_string(&mut s)?;
        s.pop(); // pop '\n'
        Ok(s)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        let typ = unsafe { ffi::CStr::from_ptr(get_os_release() as *const c_char).to_bytes() };
        Ok(String::from_utf8_lossy(typ).into_owned())
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get cpu num quantity.
///
/// Notice, it returns the logical cpu quantity.
pub fn cpu_num() -> Result<u32, Error> {
    if cfg!(unix) || cfg!(windows) {
        unsafe { Ok(get_cpu_num()) }
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    if cfg!(target_os = "linux") {
        // /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_cur_freq
        let mut s = String::new();
        File::open("/proc/cpuinfo")?.read_to_string(&mut s)?;

        s.split('\n')
            .find(|line| line.starts_with("cpu MHz"))
            .and_then(|line| line.split(':').last())
            .and_then(|val| val.trim().parse::<f64>().ok())
            .map(|speed| speed as u64)
            .ok_or(Error::Unknown)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        unsafe { Ok(get_cpu_speed()) }
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get system load average value.
///
/// Notice, on windows, one/five/fifteen of the LoadAvg returned are the current load.
pub fn loadavg() -> Result<LoadAvg, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/loadavg")?.read_to_string(&mut s)?;
        let loads = s.trim().split(' ')
            .take(3)
            .map(|val| val.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        Ok(LoadAvg {
            one: loads[0],
            five: loads[1],
            fifteen: loads[2],
        })
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_loadavg() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get current processes quantity.
///
/// Notice, it temporarily does not support Windows.
pub fn proc_total() -> Result<u64, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/loadavg")?.read_to_string(&mut s)?;
        s.split(' ')
            .nth(3)
            .and_then(|val| val.split('/').last())
            .and_then(|val| val.parse::<u64>().ok())
            .ok_or(Error::Unknown)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_proc_total() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}


/// Get memory information.
///
/// On Mac OS X and Windows, the buffers and cached variables of the MemInfo returned are zero.
pub fn mem_info() -> Result<MemInfo, Error> {
    if cfg!(target_os = "linux") {
        let mut s = String::new();
        File::open("/proc/meminfo")?.read_to_string(&mut s)?;
        let mut meminfo_hashmap = HashMap::new();
        for line in s.lines() {
            let mut split_line = line.split_whitespace();
            let label = split_line.next();
            let value = split_line.next();
            if value.is_some() && label.is_some() {
                let label = label.unwrap().split(":").nth(0).ok_or(Error::Unknown)?;
                let value = value.unwrap().parse::<u64>().ok().ok_or(Error::Unknown)?;
                meminfo_hashmap.insert(label, value);
            }
        }
        Ok(MemInfo {
            total: *meminfo_hashmap.get("MemTotal").ok_or(Error::Unknown)?,
            free: *meminfo_hashmap.get("MemFree").ok_or(Error::Unknown)?,
            avail: *meminfo_hashmap.get("MemAvailable").ok_or(Error::Unknown)?,
            buffers: *meminfo_hashmap.get("Buffers").ok_or(Error::Unknown)?,
            cached: *meminfo_hashmap.get("Cached").ok_or(Error::Unknown)?,
            swap_total: *meminfo_hashmap.get("SwapTotal").ok_or(Error::Unknown)?,
            swap_free: *meminfo_hashmap.get("SwapFree").ok_or(Error::Unknown)?,
        })
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_mem_info() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get disk information.
///
/// Notice, it just calculate current disk on Windows.
pub fn disk_info() -> Result<DiskInfo, Error> {
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_disk_info() })
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get hostname.
pub fn hostname() -> Result<String, Error> {
    use std::process::Command;
    if cfg!(unix) {
        Command::new("hostname")
            .output()
            .map_err(|e| Error::ExecFailed(e))
            .map(|output| String::from_utf8(output.stdout).unwrap().trim().to_string())
    } else if cfg!(windows) {
        Command::new("hostname")
            .output()
            .map_err(|e| Error::ExecFailed(e))
            .map(|output| String::from_utf8(output.stdout).unwrap().trim().to_string())
    } else {
        Err(Error::UnsupportedSystem)
    }
}

/// Get system boottime
#[cfg(not(windows))]
pub fn boottime() -> Result<timeval, Error> {
    let mut bt = timeval {
        tv_sec: 0,
        tv_usec: 0
    };

    #[cfg(target_os = "linux")]
    {
        let mut s = String::new();
        File::open("/proc/uptime")?.read_to_string(&mut s)?;
        let secs = s.trim().split(' ')
            .take(2)
            .map(|val| val.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        bt.tv_sec = secs[0] as libc::time_t;
        bt.tv_usec = secs[1] as libc::suseconds_t;
    }
    #[cfg(target_os = "macos")]
    {
        let mut mib = [MAC_CTL_KERN, MAC_KERN_BOOTTIME];
        let mut size: libc::size_t = size_of_val(&bt) as libc::size_t;
        unsafe {
            sysctl(&mut mib[0], 2,
                   &mut bt as *mut timeval as *mut libc::c_void,
                   &mut size, null_mut(), 0);
        }
    }

    Ok(bt)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_os_type() {
        let typ = os_type().unwrap();
        assert!(typ.len() > 0);
        println!("os_type(): {}", typ);
    }

    #[test]
    pub fn test_os_release() {
        let release = os_release().unwrap();
        assert!(release.len() > 0);
        println!("os_release(): {}", release);
    }

    #[test]
    pub fn test_cpu_num() {
        let num = cpu_num().unwrap();
        assert!(num > 0);
        println!("cpu_num(): {}", num);
    }

    #[test]
    pub fn test_cpu_speed() {
        let speed = cpu_speed().unwrap();
        assert!(speed > 0);
        println!("cpu_speed(): {}", speed);
    }

    #[test]
    pub fn test_loadavg() {
        let load = loadavg().unwrap();
        println!("loadavg(): {:?}", load);
    }

    #[test]
    pub fn test_proc_total() {
        let procs = proc_total().unwrap();
        assert!(procs > 0);
        println!("proc_total(): {}", procs);
    }

    #[test]
    pub fn test_mem_info() {
        let mem = mem_info().unwrap();
        assert!(mem.total > 0);
        println!("mem_info(): {:?}", mem);
    }

    #[test]
    pub fn test_disk_info() {
        let info = disk_info().unwrap();
        println!("disk_info(): {:?}", info);
    }

    #[test]
    pub fn test_hostname() {
        let host = hostname().unwrap();
        assert!(host.len() > 0);
        println!("hostname(): {}", host);
    }

    #[test]
    pub fn test_boottime() {
        let bt = boottime().unwrap();
        println!("boottime(): {} {}", bt.tv_sec, bt.tv_usec);
        assert!(bt.tv_sec > 0 || bt.tv_usec > 0);
    }
}
