/*
 * ffi.rs
 */


use libc::{c_char, c_int};

/* error codes */
pub const XMP_END: i32 = 1;
pub const XMP_ERROR_INTERNAL: i32 = 2;
pub const XMP_ERROR_FORMAT: i32 = 3;
pub const XMP_ERROR_LOAD: i32 = 4;
pub const XMP_ERROR_DEPACK: i32 = 5;
pub const XMP_ERROR_SYSTEM: i32 = 6;
pub const XMP_ERROR_INVALID: i32 = 7;
pub const XMP_ERROR_STATE: i32 = 8;

pub const XMP_NAME_SIZE: usize = 64;

#[repr(C)]
pub struct xmp_test_info {
    pub t_name: [c_char; XMP_NAME_SIZE],
    pub t_type: [c_char; XMP_NAME_SIZE]
}

pub type xmp_context = *mut c_char;

#[link(name = "xmp")]
extern {
    pub fn xmp_test_module(path: *const c_char, info: *mut xmp_test_info) -> c_int;
    pub fn xmp_create_context() -> *mut xmp_context;
    pub fn xmp_free_context(context: *mut xmp_context) -> ();
}
