#![feature(core)]
#![feature(std_misc)]
use std::ffi;

#[repr(C)]
pub struct LoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64
}

#[repr(C)]
pub struct MemInfo {
    pub total: u64,
    pub avail: u64,
    pub free:  u64,

    pub buffers: u64,
    pub cached:  u64,

    pub swap_total: u64,
    pub swap_free:  u64
}

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

pub fn os_type() -> Result<String, String> {
    if cfg!(target_os = "linux") ||
        cfg!(target_os = "macos") ||
        cfg!(target_os = "windows") {
        Ok(unsafe {
            let s = get_os_type();
            String::from_utf8_lossy(ffi::c_str_to_bytes(&s)).into_owned()
        })
    } else {
        Err("Unsupported system".to_string())
    }
}

pub fn os_release() -> Result<String, String> {
    if cfg!(target_os = "linux") ||
        cfg!(target_os = "macos") ||
        cfg!(target_os = "windows")
    {
        Ok(unsafe {
            let s = get_os_release();
            String::from_utf8_lossy(ffi::c_str_to_bytes(&s)).into_owned()
        })
    } else {
        Err("Unsupported system".to_string())
    }
}

pub fn cpu_num() -> Result<u32, String> {
    if cfg!(target_os = "linux") {
        // TODO
        Ok(4) 
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_cpu_num() })
    } else {
        Err("Unsupported system".to_string())
    }
}

pub fn cpu_speed() -> Result<u64, String> {
    if cfg!(target_os = "linux") {
        // TODO
        Ok(2500)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_cpu_speed() })
    } else {
        Err("Unsupported system".to_string())
    }
}

pub fn loadavg() -> Result<LoadAvg, String> {
    if cfg!(target_os = "linux") {
            // TODO
            Ok(LoadAvg { one: 0.0, five: 0.0, fifteen: 0.0} )
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_loadavg() })
    } else {
        Err("Unsupported system".to_string())
    }
}

pub fn proc_total() -> Result<u64, String> {
    if cfg!(target_os = "linux") {
            // TODO
            Ok(100)
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_proc_total() })
    } else {
        Err("Unsupported system".to_string())
    }
}

pub fn mem_info() -> Result<MemInfo, String> {
    if cfg!(target_os = "linux") {
            // TODO
        Ok(MemInfo{ total: 0, avail: 0, free: 0, buffers: 0, cached: 0,
                    swap_total: 0, swap_free: 0})
    } else if cfg!(target_os = "macos") || cfg!(target_os = "windows") {
        Ok(unsafe { get_mem_info() })
    } else {
        Err("Unsupported system".to_string())
    }
}

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
fn test() {
    assert_eq!("Darwin", os_type().unwrap());
    assert_eq!("14.1.0", os_release().unwrap());
    
    assert_eq!(4, cpu_num().unwrap());
    cpu_speed();

    loadavg();
    proc_total();

    mem_info();
    disk_info();
}

