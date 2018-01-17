# libc_mallocator

[![Build Status](https://travis-ci.org/gnzlbg/libc_mallocator.svg?branch=master)](https://travis-ci.org/gnzlbg/libc_mallocator)

[Documentation](https://docs.rs/libc_mallocator)

A Rust allocator (unstable API) that links to libc and forces all Rust allocations to use the system's malloc-based allocator.

Usage:

```toml
# Cargo.toml
[dependencies]
libc_mallocator = "0.1"
```

Rust:

```rust
#![feature(global_allocator)]
extern crate libc_mallocator;

#[global_allocator]
static ALLOC: jemallocator::LibcMalloc = libc_mallocator::LibcMalloc;
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
