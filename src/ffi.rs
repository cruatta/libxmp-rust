/*
 * ffi.rs
 */


use libc::{c_void, c_char, c_int, c_uint, c_short, c_uchar};

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

pub const XMP_MAX_CHANNELS: usize = 64;

#[repr(C)]
pub struct xmp_test_info {
    pub t_name: [c_char; XMP_NAME_SIZE],
    pub t_type: [c_char; XMP_NAME_SIZE]
}

#[repr(C)]
pub struct xmp_event {
    note: c_uchar,
    ins: c_uchar,
    vol: c_uchar,
    fxt: c_uchar,
    fxp: c_uchar,
    f2t: c_uchar,
    f2p: c_uchar,
    flag: c_uchar
}

#[repr(C)]
pub struct xmp_channel_info {
    pub period: c_uint,
    pub position: c_uint,
    pub pitchbend: c_short,
    pub note: c_uchar,
    pub instrument: c_uchar,
    pub sample: c_uchar,
    pub volume: c_uchar,
    pub pan: c_uchar,
    pub reserved: c_uchar,
    pub event: xmp_event
}


#[repr(C)]
pub struct xmp_frame_info {
    pub pos: c_int,
    pub pattern: c_int,
    pub row: c_int,
    pub num_rows: c_int,
    pub frame: c_int,
    pub speed: c_int,
    pub bpm: c_int,
    pub time: c_int,
    pub total_time: c_int,
    pub frame_time: c_int,
    pub buffer: *const c_void,
    pub buffer_size: c_int,
    pub total_size: c_int,
    pub volume: c_int,
    pub loop_count: c_int,
    pub virt_channels: c_int,
    pub virt_used: c_int,
    pub sequence: c_int,
    pub channel_info: [xmp_channel_info; XMP_MAX_CHANNELS]
}

pub type xmp_context = *mut c_char;

#[link(name = "xmp")]
extern {
    pub fn xmp_test_module(path: *const c_char, info: *mut xmp_test_info) -> c_int;
    pub fn xmp_create_context() -> *mut xmp_context;
    pub fn xmp_free_context(context: *mut xmp_context) -> ();
    pub fn xmp_load_module(context: *mut xmp_context, path: *const c_char) -> c_int;
    pub fn xmp_get_frame_info(context: *mut xmp_context, module_info: *const xmp_frame_info);
}
 
