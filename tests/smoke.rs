#![feature(global_allocator, allocator_api)]

extern crate libc_mallocator;

use libc_mallocator::LibcMalloc;
use std::heap::{Alloc, Layout};

#[global_allocator]
static A: LibcMalloc = LibcMalloc;

#[test]
fn smoke() {
    let mut a = Vec::new();
    a.push(3);
    assert_eq!(a[0], 3);
}

/// https://github.com/rust-lang/rust/issues/45955
#[test]
fn overaligned() {
    let size = 8;
    let align = 16; // greater than size
    let iterations = 100;
    unsafe {
        let pointers: Vec<_> = (0..iterations).map(|_| {
            LibcMalloc.alloc(Layout::from_size_align(size, align).unwrap()).unwrap()
        }).collect();
        for &ptr in &pointers {
            assert_eq!((ptr as usize) % align, 0, "Got a pointer less aligned than requested")
        }

        // Clean up
        for &ptr in &pointers {
            LibcMalloc.dealloc(ptr, Layout::from_size_align(size, align).unwrap())
        }
    }
}
