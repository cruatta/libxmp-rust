/*
 * ffi.rs
 *
 * libxmp v4.4.1
 */


use libc::{c_void, c_char, c_int, c_uint, c_short, c_uchar, c_long};
use std::fmt::{Debug, Formatter, Result};

#[cfg(test)]
mod tests {

    use super::*;
    use std::mem::uninitialized;
    use std::path::Path;
    use std::ffi::{ CString, CStr };

    #[test]
    fn test_ffi_get_module_info() {
        /*
        let note_names = vec![
	          "C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "
        ];
         */

        /*
        ~/l/examples (master|✚1…) $ ./showinfo test1.xm
        Name: playboy
        Type: FastTracker v2.00 XM 1.04
        Number of patterns: 13
        Number of tracks: 121
        Number of channels: 10
        Number of instruments: 10
        Number of samples: 8
        Initial speed: 4
        Initial BPM: 125
        Length in patterns: 26
        */

        let path = Path::new("./test/test1.xm");
        let p = CString::new(path.to_string_lossy().as_ref()).unwrap();
        let p_ptr = p.as_ptr();

        unsafe {

            let ctx = xmp_create_context();
            xmp_load_module(ctx, p_ptr);

            xmp_start_player(ctx, 44100, 0);

            let mut module_info: xmp_module_info = uninitialized();
            xmp_get_module_info(ctx, &mut module_info);
            let module = module_info.module;

            let m_name = (*module).m_name.as_ptr();
            let m_name = CStr::from_ptr(m_name).to_string_lossy().into_owned();
            let m_type = (*module).m_type.as_ptr();
            let m_type = CStr::from_ptr(m_type).to_string_lossy().into_owned();

            let num_pat: i32 = (*module).pat;
            let num_ins: i32 = (*module).ins;
            let num_smp: i32 = (*module).smp;
            let num_trk: i32 = (*module).trk;
            let num_chn: i32 = (*module).chn;
            let spd: i32 = (*module).spd;
            let bpm: i32 = (*module).bpm;
            let len: i32 = (*module).len;

            assert_eq!(m_name, "playboy");
            assert_eq!(m_type, "FastTracker v2.00 XM 1.04");
            assert_eq!(num_pat, 13);
            assert_eq!(num_trk, 121);
            assert_eq!(num_chn, 10);
            assert_eq!(num_ins, 10);
            assert_eq!(num_smp, 8);
            assert_eq!(spd, 4);
            assert_eq!(bpm, 125);
            assert_eq!(len, 26);

            xmp_end_player(ctx);
            xmp_release_module(ctx);
            xmp_free_context(ctx);
        }
    }
}


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

/* player modes */
pub const XMP_MODE_AUTO: usize = 0;
pub const XMP_MODE_MOD: usize = 1;
pub const XMP_MODE_NOISETRACKER: usize = 2;
pub const XMP_MODE_PROTRACKER: usize = 3;
pub const XMP_MODE_S3M: usize = 4;
pub const XMP_MODE_ST3: usize = 5;
pub const XMP_MODE_ST3GUS: usize = 6;
pub const XMP_MODE_XM: usize = 7;
pub const XMP_MODE_FT2: usize = 8;
pub const XMP_MODE_IT: usize = 9;
pub const XMP_MODE_ITSMP: usize = 10;

/* limits */
pub const XMP_MAX_KEYS: usize = 121;
pub const XMP_MAX_ENV_POINTS: usize = 32;
pub const XMP_MAX_MOD_LENGTH: usize = 256;
pub const XMP_MAX_CHANNELS: usize = 64;
pub const XMP_MAX_SRATE: usize = 49170;
pub const XMP_MIN_SRATE: usize = 4000;
pub const XMP_MIN_BPM: usize = 20;

/* frame rate = (50 * bpm / 125) Hz */
/* frame size = (sampling rate * channels * size) / frame rate */
pub const XMP_MAX_FRAMESIZE: usize = 5 * XMP_MAX_SRATE * 2 / XMP_MIN_BPM;


#[repr(C)]
pub struct xmp_test_info {
    pub t_name: [c_char; XMP_NAME_SIZE],
    pub t_type: [c_char; XMP_NAME_SIZE]
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_event {
    pub note: c_uchar,
    pub ins: c_uchar,
    pub vol: c_uchar,
    pub fxt: c_uchar,
    pub fxp: c_uchar,
    pub f2t: c_uchar,
    pub f2p: c_uchar,
    pub flag: c_uchar
}


#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub struct xmp_sequence {
    pub entry_point: c_int,
    pub duration: c_int
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_pattern {
	  pub rows: c_int,
	  pub index: [c_int; 1]
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_channel {
    pub pan: c_int,
    pub vol: c_int,
    pub flg: c_int
}

#[repr(C)]
pub struct xmp_envelope {
    pub flg: c_int,
    pub npt: c_int,
    pub scl: c_int,
    pub sus: c_int,
    pub sue: c_int,
    pub lps: c_int,
    pub lpe: c_int,
    pub data: [c_short; XMP_MAX_ENV_POINTS * 2]
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_track {
    pub rows: c_int,
    pub event: [xmp_event; 1]
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_sample {
    pub name: [c_char; 32],
    pub len: c_int,
    pub lps: c_int,
    pub lpe: c_int,
    pub flg: c_int,
    pub data: *const c_uchar
}


#[repr(C)]
pub struct xmp_instrument {
    pub name: [c_char; 32],
    pub vol: c_int,
    pub nsm: c_int,
    pub rls: c_int,
    pub aei: xmp_envelope,
    pub pei: xmp_envelope,
    pub fei: xmp_envelope,

    pub map: [xmp_key; XMP_MAX_KEYS],

    pub extra: *const c_void,
    pub sub: *const xmp_sub_instrument
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_sub_instrument {
    pub vol: c_int,
    pub gvl: c_int,
    pub pan: c_int,
    pub xpo: c_int,
    pub fin: c_int,
    pub vwf: c_int,
    pub vde: c_int,
    pub vra: c_int,
    pub vsw: c_int,
    pub rvv: c_int,
    pub sid: c_int,
    pub nna: c_int,
    pub dct: c_int,
    pub dca: c_int,
    pub ifc: c_int,
    pub ifr: c_int
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_key {
    pub ins: c_uchar,
    pub xpo: c_char
}

#[repr(C)]
pub struct xmp_module {
    pub m_name: [c_char; XMP_NAME_SIZE],
    pub m_type: [c_char; XMP_NAME_SIZE],
    pub pat: c_int,
    pub trk: c_int,
    pub chn: c_int,
    pub ins: c_int,
    pub smp: c_int,
    pub spd: c_int,
    pub bpm: c_int,
    pub len: c_int,
    pub rst: c_int,
    pub gvl: c_int,

    pub xxp: *const *const xmp_pattern,
    pub xxt: *const *const xmp_track,
    pub xxi: *const *const xmp_instrument,
    pub xxs: *const *const xmp_sample,
    pub xxc: [xmp_channel; XMP_MAX_CHANNELS],
    pub xxo: [c_uchar; XMP_MAX_MOD_LENGTH]
}

#[derive(Debug)]
#[repr(C)]
pub struct xmp_module_info {
    pub md5: [c_uchar; 16],
    pub vol_base: c_int,
    pub module: *const xmp_module,
    pub comment: *const c_char,
    pub num_sequences: c_int,
    pub xmp_sequence: *const xmp_sequence
}

pub type xmp_context = *mut c_char;


#[link(name = "xmp")]
extern {
    //info
    pub fn xmp_get_format_list() -> *const *const c_char;

    // context
    pub fn xmp_create_context() -> xmp_context;
    pub fn xmp_free_context(c: xmp_context) -> c_void;

    // load
    pub fn xmp_test_module(path: *const c_char, test_info: *mut xmp_test_info) -> c_int;
    pub fn xmp_load_module(c: xmp_context, path: *const c_char) -> c_int;
    pub fn xmp_scan_module(c: xmp_context) -> c_void;
    pub fn xmp_release_module(c: xmp_context) -> c_void;
    pub fn xmp_load_module_from_memory(c: xmp_context, mem: *mut c_void, size: c_long) -> c_int;
    pub fn xmp_load_module_from_file(c: xmp_context, f: *mut c_void, size: c_long) -> c_int;
    pub fn xmp_get_module_info(c: xmp_context, module_info: *mut xmp_module_info) -> c_void;

    //play
    pub fn xmp_start_player(c: xmp_context, rate: c_int, format: c_int) -> c_int;
    pub fn xmp_play_frame(c: xmp_context) -> c_int;
    pub fn xmp_play_buffer(c: xmp_context, out_buffer: *mut c_void, size: c_int, b_loop: c_int) -> c_int;
    pub fn xmp_get_frame_info(c: xmp_context, info: *mut xmp_frame_info) -> c_void;
    pub fn xmp_end_player(c: xmp_context) -> c_void;

    //control
    pub fn xmp_next_position(c: xmp_context) -> c_int;
    pub fn xmp_prev_position(c: xmp_context) -> c_int;
    pub fn xmp_set_position(c: xmp_context, pos: c_int) -> c_int;
    pub fn xmp_stop_module(c: xmp_context) -> c_void;
    pub fn xmp_restart_module(c: xmp_context) -> c_void;
    pub fn xmp_seek_time(c: xmp_context) -> c_int;
    pub fn xmp_channel_mute(c: xmp_context, chn: c_int, status: c_int) -> c_int;
    pub fn xmp_channel_vol(c: xmp_context, chn: c_int, vol: c_int) -> c_int;
    pub fn xmp_inject_event(c: xmp_context, chn: c_int, event: *mut xmp_event) -> c_void;

    //params
    pub fn xmp_set_player(c: xmp_context, param: c_int, val: c_int) -> c_int;
    pub fn xmp_get_player(c: xmp_context, param: c_int) -> c_int;
    pub fn xmp_set_instrument_path(c: xmp_context, path: *const c_char) -> c_int;

    //smix
    pub fn xmp_start_smix(c: xmp_context, nch: c_int, nsmp: c_int) -> c_int;
    pub fn xmp_end_smix(c: xmp_context) -> c_void;
    pub fn xmp_smix_play_instrument(c: xmp_context, ins: c_int, note: c_int, vol: c_int, chn: c_int) -> c_int;
    pub fn xmp_smix_play_sample(c: xmp_context, ins: c_int, vol: c_int, chn: c_int) -> c_int;
    pub fn xmp_smix_channel_pan(c: xmp_context, chn: c_int, pan: c_int) -> c_int;
    pub fn xmp_smix_load_sample(c: xmp_context, num: c_int, path: *const c_char) -> c_int;
    pub fn xmp_smix_release_sample(c: xmp_context, num: c_int) -> c_int;
}


