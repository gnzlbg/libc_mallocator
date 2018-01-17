//! Bindings for libc's malloc as an allocator
//!
//! This crate exports one type, `LibcMalloc`, which implements the `Alloc` trait
//! and is suitable both as a memory allocator and as a global allocator.

#![feature(allocator_api, core_intrinsics)]
#![deny(missing_docs)]

extern crate libc;

use std::{intrinsics}; //, mem, ptr};
use std::heap::{Alloc, Layout, AllocErr}; //, Excess, CannotReallocInPlace, AllocErr, System};

use libc::{c_void};

/// Handle to the libc malloc-based allocator.
///
/// This type and a reference to this type both implement the `Alloc` trait,
/// allowing using it both in collections and as a global allocator.
pub struct LibcMalloc;

unsafe impl Alloc for LibcMalloc {
    #[inline]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        (&*self).alloc(layout)
    }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        (&*self).dealloc(ptr, layout)
    }
}

unsafe impl<'a> Alloc for &'a LibcMalloc {
    #[inline]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let ptr = libc::malloc(layout.size());
        if intrinsics::unlikely(ptr.is_null()) {
            Err(AllocErr::Exhausted { request: layout })
        } else {
            Ok(ptr as *mut u8)
        }
    }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut c_void)
    }
}
