//load.rs

use error::*;
use context::*;
use ffi::{XMP_MAX_SRATE, XMP_MIN_SRATE};

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

impl Format {

    pub fn from(format: usize) -> Format {
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

    pub fn to_i32(self) -> i32 {
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

pub type FrameInfo = ();

pub fn start_player(c: &Context, rate: Rate, format: Format) -> Result<(), XmpError> {
   Ok(())
}

pub fn play_frame(c: &Context) -> Result<(), XmpError> {
   Ok(())
}

pub fn get_frame_info(c: &Context) -> Result<FrameInfo, XmpError> {
    Ok(())
}

pub fn end_player(c: &Context) -> Result<(), XmpError> {
    Ok(())
}
