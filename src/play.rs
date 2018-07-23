//load.rs

use error::*;
use context::*;
use ffi::{XMP_MAX_SRATE, XMP_MIN_SRATE};
use ffi::{xmp_frame_info, xmp_get_frame_info, xmp_start_player, xmp_play_frame, xmp_end_player};
use std::mem::uninitialized;

pub struct Rate {
    rate: i32
}


impl Rate {

    pub fn new(rate: i32) -> Rate {
        if (rate as usize) < XMP_MAX_SRATE || (rate as usize) > XMP_MIN_SRATE {
            panic!("rate outside of min and max bounds");
        }
        Rate { rate }
    }
}

impl Into<i32> for Rate {
    fn into(self) -> i32 {
        self.rate
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Format {
    Auto,
    Mod,
    Noisetracker,
    Protracker,
    S3M,
    ST3,
    ST3GUS,
    XM,
    FT2,
    IT,
    ITSMP,
    BadFormat
}

impl Into<i32> for Format {
    fn into(self) -> i32 {
        match self {
            Format::Auto => 0,
            Format::Mod => 1,
            Format::Noisetracker => 2,
            Format::Protracker => 3,
            Format::S3M => 4,
            Format::ST3 => 5,
            Format::ST3GUS => 6,
            Format::XM => 7,
            Format::FT2 => 8,
            Format::IT => 9,
            Format::ITSMP => 10,
            _ => -1
        }
    }
}

impl From<i32> for Format {
    fn from(format: i32) -> Self {
        match format {
            0 => Format::Auto,
            1 => Format::Mod,
            2 => Format::Noisetracker,
            3 => Format::Protracker,
            4 => Format::S3M,
            5 => Format::ST3,
            6 => Format::ST3GUS,
            7 => Format::XM,
            8 => Format::FT2,
            9 => Format::IT,
            10 => Format::ITSMP,
            _ => Format::BadFormat
        }
    }
}

pub type FrameInfo = xmp_frame_info;

pub fn start_player(c: &Context, rate: Rate, format: Format) -> Result<(), XmpError> {
    let ret = unsafe {
        xmp_start_player(c.state, rate.into(), format.into())
    };

    if ret != 0 {
        return Err(XmpError::new(&format!("xmp_start_player call failed with code: {}", ret), ErrorKind::from_xmp(ret)));
    };

    Ok(())
}

pub fn play_frame(c: &Context) -> Result<(), XmpError> {
    let ret = unsafe {
        xmp_play_frame(c.state)
    };

    if ret != 0 {
        return Err(XmpError::new(&format!("xmp_play_frame call failed with code: {}", ret), ErrorKind::from_xmp(ret)));
    };

    Ok(())

}

#[cfg(test)]
mod get_frame_info {

    use super::*;

    /*
    #[test]
    fn test_load_module() {
        let path = Path::new("./test/test1.xm");

        let context = Context::new();

        if let Ok(x) = load_module(&context, &path) {
            assert_eq!(x, ());
        }

        if let Ok(x) = load_module(&context, &path) {
            assert_eq!(x, ());
        }

    }
    */

    #[test]
    fn test_get_frame_info_not_loaded() {
        let context = Context::new();

        if let Err(x) = get_frame_info(&context) {
            assert_eq!(x.kind, ErrorKind::SelfType(SelfErrorKind::Other))
        }
    }
}
pub fn get_frame_info(c: &Context) -> Result<FrameInfo, XmpError> {
    let frame_info  = unsafe {
        let mut frame_info: xmp_frame_info = uninitialized();
        xmp_get_frame_info(c.state, &mut frame_info);
        frame_info
    };

    let x = &frame_info as *const xmp_frame_info;

    unsafe {
        match x.as_ref() {
            Some(_) => Ok(frame_info),
            _ => return Err(XmpError::new("xmp_get_frame_info call failed", ErrorKind::SelfType(SelfErrorKind::Other)))
        }
    }
}

pub fn end_player(c: &Context) -> Result<(), XmpError> {
    unsafe {
        xmp_end_player(c.state)
    };

    Ok(())
}
