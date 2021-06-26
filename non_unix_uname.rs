use super::Error;

/// On non-unix systems, we emulate the uname data structure.
/// 
/// Fields can be accessed through methods.
///
/// Currently, this doesn't support `machine`.
pub struct Info {
    sysname: String,
    nodename: String,
    release: String,
}

impl Info {
    pub(crate) fn new() -> Result<Info, Error> {
        let release = super::os_release()?;
        Ok(Info {
            sysname: super::os_type()?,
            nodename: super::hostname()?,
            release,
        })
    }

    /// Kernel Name, for example "Linux".
    pub fn sysname(&self) -> Result<&str, Error> {
        Ok(&self.sysname)
    }
    /// Network node hostname, this is usually the same as the hostname.
    pub fn nodename(&self) -> Result<&str, Error> {
        Ok(&self.nodename)
    }
    /// Kernel release, for example "5.10.4-arch2-1".
    pub fn release(&self) -> Result<&str, Error> {
        Ok(&self.release)
    }
    /// Kernel version, for example "#1 SMP PREEMPT Fri, 01 Jan 2021 05:29:53 +0000".
    ///
    /// On non-unix systems, this is the same as `self.release()`.
    pub fn version(&self) -> Result<&str, Error> {
        Ok(&self.release)
    }
    /// Machine hardware name, for example "x86_64".
    ///
    /// Note that this isn't yet supported on non-unix systems.
    pub fn machine(&self) -> Result<&str, Error> {
        Err(Error::UnsupportedSystem)
    }
}
