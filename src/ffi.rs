/*
 * ffi.rs
 */

use libc::{c_char, c_int};


pub const XMP_NAME_SIZE: usize = 64;

#[repr(C)]
pub struct xmp_test_info {
    pub module_name: [c_char; XMP_NAME_SIZE],
    pub module_type: [c_char; XMP_NAME_SIZE]
}

#[link(name = "xmp")]
extern {
    pub fn xmp_test_module(path: *const c_char, info: *mut xmp_test_info) -> c_int;
}
