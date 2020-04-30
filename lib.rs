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
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::os::raw::c_char;
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
use std::os::raw::{c_int, c_double};

#[cfg(target_os = "macos")]
use libc::sysctl;
#[cfg(target_os = "macos")]
use std::mem::size_of_val;
#[cfg(target_os = "macos")]
use std::ptr::null_mut;
#[cfg(not(target_os = "windows"))]
use libc::timeval;
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
use std::time::SystemTime;
#[cfg(target_os = "linux")]
use std::collections::HashMap;

#[cfg(any(target_os = "solaris", target_os = "illumos"))]
mod kstat;

#[cfg(target_os = "macos")]
static MAC_CTL_KERN: libc::c_int = 1;
#[cfg(target_os = "macos")]
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

/// The os release info of Linux
#[derive(Debug)]
#[derive(Default)]
pub struct LinuxOSReleaseInfo {
    pub id: Option<String>,
    pub id_like: Option<String>,
    pub name: Option<String>,
    pub pretty_name: Option<String>,

    pub version: Option<String>,
    pub version_id: Option<String>,
    pub version_codename: Option<String>,

    pub ansi_color: Option<String>,
    pub cpe_name: Option<String>,
    pub build_id: Option<String>,
    pub variant: Option<String>,
    pub variant_id: Option<String>,

    pub home_url: Option<String>,
    pub bug_report_url: Option<String>,
    pub support_url: Option<String>,
    pub documentation_url: Option<String>,
    pub logo: Option<String>,
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
    SystemTime(std::time::SystemTimeError),
    General(String),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnsupportedSystem => write!(fmt, "System is not supported"),
            ExecFailed(ref e) => write!(fmt, "Execution failed: {}", e),
            IO(ref e) => write!(fmt, "IO error: {}", e),
            SystemTime(ref e) => write!(fmt, "System time error: {}", e),
            General(ref e) => write!(fmt, "Error: {}", e),
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
            SystemTime(_) => "system time",
            General(_) => "general error",
            Unknown => "unknown error",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        use self::Error::*;
        match *self {
            UnsupportedSystem => None,
            ExecFailed(ref e) => Some(e),
            IO(ref e) => Some(e),
            SystemTime(ref e) => Some(e),
            General(_) => None,
            Unknown => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Error {
        Error::SystemTime(e)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Error {
        Error::General(e.to_string())
    }
}

extern "C" {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn get_os_type() -> *const i8;
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn get_os_release() -> *const i8;

    #[cfg(all(not(any(target_os = "solaris", target_os = "illumos")), any(unix, windows)))]
    fn get_cpu_num() -> u32;
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn get_cpu_speed() -> u64;

    #[cfg(target_os = "windows")]
    fn get_loadavg() -> LoadAvg;
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn get_proc_total() -> u64;

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn get_mem_info() -> MemInfo;
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    fn get_disk_info() -> DiskInfo;
}


/// Get operation system type.
///
/// Such as "Linux", "Darwin", "Windows".
pub fn os_type() -> Result<String, Error> {
    #[cfg(target_os = "linux")]
    {
        let mut s = String::new();
        File::open("/proc/sys/kernel/ostype")?.read_to_string(&mut s)?;
        s.pop(); // pop '\n'
        Ok(s)
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let typ = unsafe { ffi::CStr::from_ptr(get_os_type() as *const c_char).to_bytes() };
        Ok(String::from_utf8_lossy(typ).into_owned())
    }
    #[cfg(target_os = "solaris")]
    {
        Ok("solaris".to_string())
    }
    #[cfg(target_os = "illumos")]
    {
        Ok("illumos".to_string())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows", target_os = "solaris", target_os = "illumos")))]
    {
        Err(Error::UnsupportedSystem)
    }
}

/// Get operation system release version.
///
/// Such as "3.19.0-gentoo"
pub fn os_release() -> Result<String, Error> {
    #[cfg(target_os = "linux")]
    {
        let mut s = String::new();
        File::open("/proc/sys/kernel/osrelease")?.read_to_string(&mut s)?;
        s.pop(); // pop '\n'
        Ok(s)
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let typ = unsafe { ffi::CStr::from_ptr(get_os_release() as *const c_char).to_bytes() };
        Ok(String::from_utf8_lossy(typ).into_owned())
    }
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        let release: Option<String> = unsafe {
            let mut name: libc::utsname = std::mem::zeroed();
            if libc::uname(&mut name) < 0 {
                None
            } else {
                let cstr = std::ffi::CStr::from_ptr(name.release.as_mut_ptr());
                Some(cstr.to_string_lossy().to_string())
            }
        };
        match release {
            None => Err(Error::Unknown),
            Some(release) => Ok(release),
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows", target_os = "solaris", target_os = "illumos")))]
    {
        Err(Error::UnsupportedSystem)
    }
}

/// Get the os release note of Linux
///
/// Information in /etc/os-release, such as name and version of distribution.
pub fn linux_os_release() -> Result<LinuxOSReleaseInfo, Error> {
    if !cfg!(target_os = "linux") {
        return Err(Error::UnsupportedSystem);
    }

    let mut s = String::new();
    File::open("/etc/os-release")?.read_to_string(&mut s)?;

    let mut info: LinuxOSReleaseInfo = Default::default();
    for l in s.split('\n') {
        match parse_line_for_linux_os_release(l.trim().to_string()) {
            Some((key, value)) =>
                match (key.as_ref(), value) {
                    ("ID", val) => info.id = Some(val),
                    ("ID_LIKE", val) => info.id_like = Some(val),
                    ("NAME", val) => info.name = Some(val),
                    ("PRETTY_NAME", val) => info.pretty_name = Some(val),

                    ("VERSION", val) => info.version = Some(val),
                    ("VERSION_ID", val) => info.version_id = Some(val),
                    ("VERSION_CODENAME", val) => info.version_codename = Some(val),

                    ("ANSI_COLOR", val) => info.ansi_color = Some(val),
                    ("CPE_NAME", val) => info.cpe_name = Some(val),
                    ("BUILD_ID", val) => info.build_id = Some(val),
                    ("VARIANT", val) => info.variant = Some(val),
                    ("VARIANT_ID", val) => info.variant_id = Some(val),

                    ("HOME_URL", val) => info.home_url = Some(val),
                    ("BUG_REPORT_URL", val) => info.bug_report_url = Some(val),
                    ("SUPPORT_URL", val) => info.support_url = Some(val),
                    ("DOCUMENTATION_URL", val) => info.documentation_url = Some(val),
                    ("LOGO", val) => info.logo = Some(val),
                    _ => {}
                }
            None => {}
        }
    }

    Ok(info)
}

fn parse_line_for_linux_os_release(l: String) -> Option<(String, String)> {
    let words: Vec<&str> = l.splitn(2, '=').collect();
    if words.len() < 2 {
        return None
    }
    let mut trim_value = String::from(words[1]);

    if trim_value.starts_with('"') {
        trim_value.remove(0);
    }
    if trim_value.ends_with('"') {
        trim_value.remove(trim_value.len()-1);
    }

    return Some((String::from(words[0]), trim_value))
}

/// Get cpu num quantity.
///
/// Notice, it returns the logical cpu quantity.
pub fn cpu_num() -> Result<u32, Error> {
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        let ret = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
        if ret < 1 || ret > std::u32::MAX as i64 {
            Err(Error::Unknown)
        } else {
            Ok(ret as u32)
        }
    }
    #[cfg(all(not(any(target_os = "solaris", target_os = "illumos")), any(unix, windows)))]
    {
        unsafe { Ok(get_cpu_num()) }
    }
    #[cfg(not(any(target_os = "solaris", target_os = "illumos", unix, windows)))]
    {
        Err(Error::UnsupportedSystem)
    }
}

/// Get cpu speed.
///
/// Such as 2500, that is 2500 MHz.
pub fn cpu_speed() -> Result<u64, Error> {
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
       Ok(kstat::cpu_mhz()?)
    }
    #[cfg(target_os = "linux")]
    {
        // /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_cur_freq
        let mut s = String::new();
        File::open("/proc/cpuinfo")?.read_to_string(&mut s)?;

        let find_cpu_mhz = s.split('\n').find(|line|
            line.starts_with("cpu MHz\t") ||
                line.starts_with("BogoMIPS") ||
                line.starts_with("clock\t") ||
                line.starts_with("bogomips per cpu")
        );

        find_cpu_mhz.and_then(|line| line.split(':').last())
            .and_then(|val| val.replace("MHz", "").trim().parse::<f64>().ok())
            .map(|speed| speed as u64)
            .ok_or(Error::Unknown)
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        unsafe { Ok(get_cpu_speed()) }
    }
    #[cfg(not(any(target_os = "solaris", target_os = "illumos", target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(Error::UnsupportedSystem)
    }
}

/// Get system load average value.
///
/// Notice, on windows, one/five/fifteen of the LoadAvg returned are the current load.
pub fn loadavg() -> Result<LoadAvg, Error> {
    #[cfg(target_os = "linux")]
    {
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
    }
    #[cfg(any(target_os = "solaris", target_os = "illumos", target_os = "macos"))]
    {
        let mut l: [c_double; 3] = [0f64; 3];
        if unsafe { libc::getloadavg(l.as_mut_ptr(), l.len() as c_int) } < 3 {
            Err(Error::Unknown)
        } else {
            Ok(LoadAvg {
                one: l[0],
                five: l[1],
                fifteen: l[2],
            })
        }
    }
    #[cfg(any(target_os = "windows"))]
    {
        Ok(unsafe { get_loadavg() })
    }
    #[cfg(not(any(target_os = "linux", target_os = "solaris", target_os = "illumos", target_os = "macos", target_os = "windows")))]
    {
        Err(Error::UnsupportedSystem)
    }
}

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        Ok(kstat::nproc()?)
    }
    #[cfg(target_os = "linux")]
    {
        let mut s = String::new();
        File::open("/proc/loadavg")?.read_to_string(&mut s)?;
        s.split(' ')
            .nth(3)
            .and_then(|val| val.split('/').last())
            .and_then(|val| val.parse::<u64>().ok())
            .ok_or(Error::Unknown)
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        Ok(unsafe { get_proc_total() })
    }
    #[cfg(not(any(target_os = "linux", target_os = "solaris", target_os = "illumos", target_os = "macos", target_os = "windows")))]
    {
        Err(Error::UnsupportedSystem)
    }
}

#[cfg(any(target_os = "solaris", target_os = "illumos"))]
fn pagesize() -> Result<u32, Error> {
    let ret = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if ret < 1 || ret > std::u32::MAX as i64 {
        Err(Error::Unknown)
    } else {
        Ok(ret as u32)
    }
}

/// Get memory information.
///
/// On Mac OS X and Windows, the buffers and cached variables of the MemInfo returned are zero.
pub fn mem_info() -> Result<MemInfo, Error> {
    #[cfg(target_os = "linux")]
    {
        let mut s = String::new();
        File::open("/proc/meminfo")?.read_to_string(&mut s)?;
        let mut meminfo_hashmap = HashMap::new();
        for line in s.lines() {
            let mut split_line = line.split_whitespace();
            let label = split_line.next();
            let value = split_line.next();
            if value.is_some() && label.is_some() {
                let label = label.unwrap().split(':').nth(0).ok_or(Error::Unknown)?;
                let value = value.unwrap().parse::<u64>().ok().ok_or(Error::Unknown)?;
                meminfo_hashmap.insert(label, value);
            }
        }
        let total = *meminfo_hashmap.get("MemTotal").ok_or(Error::Unknown)?;
        let free = *meminfo_hashmap.get("MemFree").ok_or(Error::Unknown)?;
        let buffers = *meminfo_hashmap.get("Buffers").ok_or(Error::Unknown)?;
        let cached = *meminfo_hashmap.get("Cached").ok_or(Error::Unknown)?;
        let avail = meminfo_hashmap.get("MemAvailable").map(|v| v.clone()).or_else(|| {
            let sreclaimable = *meminfo_hashmap.get("SReclaimable")?;
            let shmem = *meminfo_hashmap.get("Shmem")?;
            Some(free + buffers + cached + sreclaimable - shmem)
        }).ok_or(Error::Unknown)?;
        let swap_total = *meminfo_hashmap.get("SwapTotal").ok_or(Error::Unknown)?;
        let swap_free = *meminfo_hashmap.get("SwapFree").ok_or(Error::Unknown)?;
        Ok(MemInfo {
            total,
            free,
            avail,
            buffers,
            cached,
            swap_total,
            swap_free,
        })
    }
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        let pagesize = pagesize()? as u64;
        let pages = kstat::pages()?;
        return Ok(MemInfo {
            total: pages.physmem * pagesize / 1024,
            avail: 0,
            free: pages.freemem * pagesize / 1024,
            cached: 0,
            buffers: 0,
            swap_total: 0,
            swap_free: 0,
        });
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        Ok(unsafe { get_mem_info() })
    }
    #[cfg(not(any(target_os = "linux", target_os = "solaris", target_os = "illumos", target_os = "macos", target_os = "windows")))]
    {
        Err(Error::UnsupportedSystem)
    }
}

/// Get disk information.
///
/// Notice, it just calculate current disk on Windows.
pub fn disk_info() -> Result<DiskInfo, Error> {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    {
        Ok(unsafe { get_disk_info() })
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(Error::UnsupportedSystem)
    }
}

/// Get hostname.
#[cfg(target_family = "unix")]
pub fn hostname() -> Result<String, Error> {
    unsafe {
        let buf_size = libc::sysconf(libc::_SC_HOST_NAME_MAX) as usize;
        let mut buf = Vec::<u8>::with_capacity(buf_size + 1);
        if libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf_size) < 0 {
            return Err(Error::IO(io::Error::last_os_error()));
        }
        let hostname_len = libc::strnlen(buf.as_ptr() as *const libc::c_char, buf_size);
        buf.set_len(hostname_len);
        Ok(ffi::CString::new(buf).unwrap().into_string().unwrap())
    }
}

#[cfg(target_family = "windows")]
pub fn hostname() -> Result<String, Error> {
    use std::process::Command;
    Command::new("hostname")
        .output()
        .map_err(Error::ExecFailed)
        .map(|output| String::from_utf8(output.stdout).unwrap().trim().to_string())
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
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        let start = kstat::boot_time()?;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let now = now.as_secs();
        if now < start {
            return Err(Error::General("time went backwards".into()));
        }
        bt.tv_sec = (now - start) as i64;
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
    #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
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
    #[cfg(not(windows))]
    pub fn test_boottime() {
        let bt = boottime().unwrap();
        println!("boottime(): {} {}", bt.tv_sec, bt.tv_usec);
        assert!(bt.tv_sec > 0 || bt.tv_usec > 0);
    }

    #[test]
    #[cfg(linux)]
    pub fn test_linux_os_release() {
        let os_release = linux_os_release().unwrap();
        println!("linux_os_release(): {:?}", os_release.name)
    }
}
