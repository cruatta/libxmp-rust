//error.rs

use std::fmt;
use std::error::Error;

use ffi::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum InternalErrorKind {
    End,
    Internal,
    Load,
    Depack,
    Invalid,
    State,
    System,
    Other,
    BadFormat
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum SelfErrorKind {
    Other
}


#[derive(PartialEq)]
#[derive(Debug)]
pub enum ErrorKind {
    SelfType(SelfErrorKind),
    InternalType(InternalErrorKind)
}

impl ErrorKind {
    pub fn from_xmp(ret: i32) -> Self {
        match ret.abs() {
            XMP_END => ErrorKind::InternalType(InternalErrorKind::End),
            XMP_ERROR_INTERNAL => ErrorKind::InternalType(InternalErrorKind::Internal),
            XMP_ERROR_FORMAT => ErrorKind::InternalType(InternalErrorKind::BadFormat),
            XMP_ERROR_LOAD => ErrorKind::InternalType(InternalErrorKind::Load),
            XMP_ERROR_DEPACK => ErrorKind::InternalType(InternalErrorKind::Depack),
            XMP_ERROR_SYSTEM => ErrorKind::InternalType(InternalErrorKind::System),
            XMP_ERROR_INVALID => ErrorKind::InternalType(InternalErrorKind::Invalid),
            XMP_ERROR_STATE => ErrorKind::InternalType(InternalErrorKind::State),
            _ => ErrorKind::InternalType(InternalErrorKind::Other)
        }
    }
}

#[derive(Debug)]
pub struct XmpError {
    pub details: String,
    pub kind: ErrorKind
}

impl XmpError {
    pub fn new(msg: &str, kind: ErrorKind) -> XmpError {
        XmpError{ details: msg.to_string(), kind }
    }
}

impl fmt::Display for XmpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for XmpError {
    fn description(&self) -> &str {
        &self.details
    }
}

