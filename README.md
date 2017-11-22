# sys-info-rs

[![Build Status](https://travis-ci.org/FillZpp/sys-info-rs.svg?branch=master)](https://travis-ci.org/FillZpp/sys-info-rs)

Get system information in Rust.

For now it supports Linux, Mac OS X and Windows.
And now it can get information of kernel/cpu/memory/disk/load/hostname and so on.

[Documentation](https://docs.rs/sys-info/0.5.3/sys_info/)

### Usage
Add this to `Cargo.toml`:

```
[dependencies]
sys-info = "*"
```

and add this to crate root:

```
extern crate sys_info;
```

