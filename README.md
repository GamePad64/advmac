# advMAC

Advanced MAC address library.

[![Crates.io](https://img.shields.io/crates/v/advmac?style=flat-square)](https://crates.io/crates/advmac)
[![docs.rs](https://img.shields.io/docsrs/advmac?style=flat-square)](https://docs.rs/advmac/latest)
[![Codecov](https://img.shields.io/codecov/c/gh/GamePad64/advmac?logo=codecov&style=flat-square)](https://codecov.io/gh/GamePad64/advmac)

# Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
advmac = "1.0.0"
```

## Features:
- EUI-48 and EUI-64.
- Extensive `no_std` support.
- `serde` support (even on `no_std`).
- `const fn` address parser with convenience macros for compile-time address handling: `mac6!`, `mac8!`.
- MAC address generation and editing.
