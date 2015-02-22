#![feature(core)]
#![feature(std_misc)]
#![feature(fs)]
#![feature(io)]

//! #Introduction
//! This crate focuses on geting system information.
//!
//! For now it supports Linux, Mac OS X and Windows.
//! And now it can get information of kernel/cpu/memory/disk/load and so on.
//!
//! If you have some advice or wish this crate supports more system or information, contact with me.

use std::ffi;
use std::io::Read;
use std::fs::File;

/// System load average value.
#[repr(C)]
pub struct LoadAvg {
    /// Average load value within one minite.
    pub one: f64,
    /// Average load value within five minites.
    pub five: f64,
    /// Average load value within fifteen minites.
    pub fifteen: f64
}

/// System memory information.
#[repr(C)]
pub struct MemInfo {
    /// Total physical memory.
    pub total: u64,
    pub free:  u64,
    pub avail: u64,

    pub buffers: u64,
    pub cached:  u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free:  u64
}

/// System momory information.
#[repr(C)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64
}

extern {
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
pub fn os_type() -> Result<String, String> {
    if cfg!(target_os = "linux") {
        let mut f = File::open("/proc/sys/kernel/ostype").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s).unwrap();
        s.pop();  // pop '\n'
        Ok(s)
    } else if cfg!(target_os = "macos") ||
        cfg!(target_os = "windows")
    {
        Ok(unsafe {
            let p = get_os_type();
            String::from_utf8_lossy(ffi::CStr::from_ptr(p).to_bytes()).into_owned()
        })
    } else {
        Err("Unsupported system".to_string())
    }
}

/// Get operation system release version.
///
/// Such as "3.19.0-gentoo"
pub fn os_release() -> Result<String, String> {
    if cfg!(target_os = "linux") {
        let mut f = File::open("/proc/sys/kernel/osrelease").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s).unwrap();
        s.pop();
        Ok(s)
    } else if cfg!(target_os = "macos") ||
        cfg!(target_os = "windows")
    {
        Ok(unsafe {
            let p = get_os_release();
            String::from_utf8_lossy(ffi::CStr::from_ptr(p).to_bytes()).into_owned()
        })
    } else {
        Err("Unsupported system".to_string())
    }
}

/// Get cpu num quantity.
///
/// Notice, it returns the logical cpu quantity.
pub fn cpu_num() -> Result<u32, String> {
    if cfg!(target_os = "linux")
        || cfg!(target_os = "macos") || cfg!(target_os = "windows")
    {
        Ok(unsafe { get_cpu_num() })
    } else {
        Err("Unsupported system".to_string())
    }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, String> {
    if cfg!(target_os = "linux") {
        // /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_cur_freq
        let mut f = File::open("/proc/cpuinfo").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s).unwrap();
        let mut lines = s.as_slice().split('\n');
        Ok({
            for _ in 0..7 {
                lines.next();
            }
            let mut words = lines.next().unwrap().split(':');
            words.next();
            let s = words.next().unwrap().trim().trim_right_matches('\n');
            s.parse::<f64>().unwrap() as u64
        })
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_cpu_speed() })
    } else {
        Err("Unsupported system".to_string())
    }
}

/// Get system load average value.
///
/// Notice, on windows, one/five/fifteen of the LoadAvg returned are the current load.
pub fn loadavg() -> Result<LoadAvg, String> {
    if cfg!(target_os = "linux") {
        let mut f = File::open("/proc/loadavg").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s).unwrap();
        let mut words = s.as_slice().split(' ');
        let one = words.next().unwrap().parse::<f64>().unwrap();
        let five = words.next().unwrap().parse::<f64>().unwrap();
        let fifteen = words.next().unwrap().parse::<f64>().unwrap();
        Ok(LoadAvg { one: one, five: five, fifteen: fifteen} )
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_loadavg() })
    } else {
        Err("Unsupported system".to_string())
    }
}

/// Get current processes quantity.
///
/// Notice, it temporarily does not support Windows.
pub fn proc_total() -> Result<u64, String> {
    if cfg!(target_os = "linux") {
        let mut f = File::open("/proc/loadavg").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s).unwrap();
        Ok({
            let mut words = s.as_slice().splitn(3, ' ');
            for _ in 0..3 {
                words.next();
            }
            let mut words = words.next().unwrap().split('/');
            words.next();
            let mut words = words.next().unwrap().split(' ');
            words.next().unwrap().parse::<u64>().unwrap()
        })
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_proc_total() })
    } else {
        Err("Unsupported system".to_string())
    }
}

// Analyse number from line.
fn get_mem_num(line: &str) -> u64 {
    let mut line = line.splitn(1, ' ');
    line.next();
    line.next().unwrap().trim_left().split(' ').next().unwrap()
        .parse::<u64>().unwrap()
}

/// Get memory information.
///
/// On Mac OS X and Windows, the buffers and cached variables of the MemInfo returned are zero.
pub fn mem_info() -> Result<MemInfo, String> {
    if cfg!(target_os = "linux") {
        let mut f = File::open("/proc/meminfo").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s).unwrap();
        let mut lines = s.as_slice().split('\n');
        let total = get_mem_num(lines.next().unwrap());
        let free = get_mem_num(lines.next().unwrap());
        let avail = get_mem_num(lines.next().unwrap());
        let buffers = get_mem_num(lines.next().unwrap());
        let cached = get_mem_num(lines.next().unwrap());
        let swap_total = {
            for _ in 0..9 {
                lines.next();
            }
            get_mem_num(lines.next().unwrap())
        };
        let swap_free = get_mem_num(lines.next().unwrap());
        
        Ok(MemInfo{ total: total, free: free, avail: avail,
                    buffers: buffers, cached: cached,
                    swap_total: swap_total, swap_free: swap_free})
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_mem_info() })
    } else {
        Err("Unsupported system".to_string())
    }
}

/// Get disk information.
///
/// Notice, it just calculate current disk on Windows.
pub fn disk_info() -> Result<DiskInfo, String> {
    if cfg!(target_os = "linux") ||
        cfg!(target_os = "macos") ||
        cfg!(target_os = "windows")
    {
        Ok(unsafe { get_disk_info() })
    } else {
        Err("Unsupported system".to_string())
    }
}


#[test]
fn test_os_type() {
    os_type().unwrap();
}

#[test]
fn test_os_release() {
    os_release().unwrap();
}

#[test]
fn test_cpu_num() {
    cpu_num().unwrap();
}

#[test]
fn test_cpu_speed() {
    cpu_speed().unwrap();
}

#[test]
fn test_loadavg() {
    loadavg().unwrap();
}

#[test]
fn test_proc_total() {
    proc_total().unwrap();
}

#[test]
fn test_mem_info() {
    mem_info().unwrap();
}

#[test]
fn test_disk_info() {
    disk_info().unwrap();
}

