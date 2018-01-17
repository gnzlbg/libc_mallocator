#![feature(global_allocator)]

extern crate libc_mallocator;
extern crate libc;

#[global_allocator]
static A: libc_mallocator::LibcMalloc = libc_mallocator::LibcMalloc;

#[test]
fn smoke() {
    unsafe {
        let ptr = libc::malloc(4);
        *(ptr as *mut u32) = 0xDECADE;
        assert_eq!(*(ptr as *mut u32), 0xDECADE);
        libc::free(ptr);
    }
}
