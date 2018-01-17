# libc_mallocator

[![Build Status](https://travis-ci.org/gnzlbg/libc_mallocator.svg?branch=master)](https://travis-ci.org/gnzlbg/libc_mallocator)

[Documentation](https://docs.rs/libc_mallocator)

> A Rust allocator (unstable API) backed by libc's malloc.


# Example

```toml
# Cargo.toml
[dependencies]
libc_mallocator = "0.1"
```

Add

```rust
#![feature(global_allocator)]
extern crate libc_mallocator;

#[global_allocator]
static ALLOC: libc_mallocator::LibcMalloc = libc_mallocator::LibcMalloc;
```

to make `libc_mallocator::LibcMalloc` the system's allocator in your program.

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
