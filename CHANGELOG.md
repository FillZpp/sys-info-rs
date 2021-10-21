# ChangeLog

## 0.9.1

- Fix iOS Support and CPU speed doesn't work on ARM64 Macs either.
- Rust Nightly fix
- Add a cast to allow building on ILP32 systems
- Prevent free swap from getting larger than total swap
- Fix compiler errors/warnings for NetBSD/arm

## 0.9.0

- Typo fixes & test fixup
- macOS: On failure copy the unknown value and null-terminate it correctly
- Fix windows-gnu build
- Support for NetBSD platform

## 0.8.0

- Fix build for target android
- add OpenBSD support
- Make get_cpu_speed arch-independent on windows
- improve get_mem_info on macos
- Make Disk Info Thread-Safe on Linux
- Loop to find max CPU speed in Windows get_cpu_speed

## 0.7.0

- FreeBSD port.

## 0.6.1

- Restore `Send` trait to `Error` for wrapping with `error-chain`
- Use cfg attribute instead of cfg! macro, which fixes Windows build errors in v0.6.0

## 0.6.0

- Support illumos and Solaris systems

## 0.5.10

- Cast gethostname() arguments to a proper type.

## 0.5.9

- Optimize getHostname for hostname command might not be installed.

## 0.5.8

- Support get os-release information for Linux #38
