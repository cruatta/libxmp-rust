//lib.rs
extern crate libc;

use std::path::Path;
use std::ffi::{ CString, CStr };
use std::fmt;
use std::error::Error;

mod ffi;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_test_module_xm() {
        let path = Path::new("./test/lol.xm");

        match test_module(&path) {
            Ok(x) => {
                assert_eq!(x.t_name, "playboy");
                assert_eq!(x.t_type, "Fast Tracker II");
            },
            Err(y) => panic!(y.details)
        }
    }

    #[test]
    fn test_test_module_dir() {
        let path = Path::new("./test/");

        if let Err(x) = test_module(&path) {
            assert_eq!(x.kind, ErrorKind::System)
        }
    }

    #[test]
    fn test_test_module_bad_path() {
        let path = Path::new("./test/bad");

        if let Err(x) = test_module(&path) {
            assert_eq!(x.kind, ErrorKind::System)
        }
    }
}

#[derive(Debug)]
pub struct TestModuleError {
    details: String,
    kind: ErrorKind
}

impl TestModuleError {
    fn new(msg: &str, kind: ErrorKind) -> TestModuleError {
        TestModuleError{details: msg.to_string(), kind }
    }
}

impl fmt::Display for TestModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for TestModuleError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct TestModuleInfo {
    t_name: String,
    t_type: String
}

#[derive(PartialEq)]
#[derive(Debug)]
enum ErrorKind {
    Internal,
    Load,
    Depack,
    Invalid,
    State,
    System,
    Other,
    BadFormat
}

pub fn test_module(path: &Path) -> Result<TestModuleInfo, TestModuleError> {
    use ffi::*;

    if path.is_dir() {
        return Err(TestModuleError::new("path is dir", ErrorKind::System))
    };


    if let Some(p) = path.to_str() {

        let test_info: xmp_test_info = unsafe {
            let mut module: xmp_test_info = std::mem::uninitialized();
            let ret = xmp_test_module(CString::new(p).unwrap().as_ptr(), &mut module);
            if ret != 0 {
                let kind = match ret.abs() {
                     XMP_END => ErrorKind::Other,
                     XMP_ERROR_INTERNAL => ErrorKind::Internal,
                     XMP_ERROR_FORMAT => ErrorKind::BadFormat,
                     XMP_ERROR_LOAD => ErrorKind::Load,
                     XMP_ERROR_DEPACK => ErrorKind::Depack,
                     XMP_ERROR_SYSTEM => ErrorKind::System,
                     XMP_ERROR_INVALID => ErrorKind::Invalid,
                     XMP_ERROR_STATE => ErrorKind::State,
                    _ => ErrorKind::Other
                };
                return Err(TestModuleError::new(&format!("xmp_test_module call failed with code: {}", ret), kind))
            };
            module
        };

        let test_info: TestModuleInfo = unsafe {
            let t_name = CStr::from_ptr(test_info.t_name.as_ptr());
            let t_name = t_name.to_owned().into_string().unwrap();
            let t_type = CStr::from_ptr(test_info.t_type.as_ptr());
            let t_type = t_type.to_owned().into_string().unwrap();
            TestModuleInfo{ t_name, t_type }
        };
        return Ok(test_info);
    }
    return Err(TestModuleError::new("path doesn't exist", ErrorKind::System));

}

