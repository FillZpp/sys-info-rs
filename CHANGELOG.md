# ChangeLog

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
