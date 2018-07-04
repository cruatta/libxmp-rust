//lib.rs
extern crate libc;

use std::path::Path;
use std::ffi::{ CString, CStr };

mod error;
mod ffi;

#[cfg(test)]
mod tests {

    use super::*;
    use super::error::*;

    #[test]
    fn test_test_module_xm0_no_title() {
        // Wiklund & Joule - Makebelieve Girl.xm
        let path = Path::new("./test/test0.xm");

        match test_module(&path) {
            Ok(x) => {
                assert_eq!(x.t_name, "");
                assert_eq!(x.t_type, "Fast Tracker II");
            },
            Err(y) => panic!(y.details)
        }
    }

    #[test]
    fn test_test_module_xm1_full() {
        //  andromeda/fairlight - playboy.xm
        let path = Path::new("./test/test1.xm");

        match test_module(&path) {
            Ok(x) => {
                assert_eq!(x.t_name, "playboy");
                assert_eq!(x.t_type, "Fast Tracker II");
            },
            Err(y) => panic!(y.details)
        }
    }

    #[test]
    fn test_test_module_mod() {
        // buzzer/zenon - dee in space v2
        let path = Path::new("./test/test0.mod");

        match test_module(&path) {
            Ok(x) => {
                assert_eq!(x.t_name, "dee in space.v2");
                assert_eq!(x.t_type, "Amiga Protracker/Compatible");
            },
            Err(y) => panic!(y.details)
        }
    }

    #[test]
    fn test_test_text_file() {
        let path = Path::new("./test/test.bad");

        if let Err(x) = test_module(&path) {
            assert_eq!(x.kind, ErrorKind::InternalType(InternalErrorKind::BadFormat))
        }
    }



    #[test]
    fn test_test_module_dir() {
        let path = Path::new("./test/");

        if let Err(x) = test_module(&path) {
            assert_eq!(x.kind, ErrorKind::InternalType(InternalErrorKind::System))
        }
    }

    #[test]
    fn test_test_module_missing_path() {
        let path = Path::new("./test/bad");

        if let Err(x) = test_module(&path) {
            assert_eq!(x.kind, ErrorKind::InternalType(InternalErrorKind::System))
        }
    }

}


pub struct TestModuleInfo {
    t_name: String,
    t_type: String
}


pub fn test_module(path: &Path) -> Result<TestModuleInfo, error::XmpError> {
    use ffi::*;

    let p = path.to_string_lossy();

    let (test_info, ret) = unsafe {
        let mut test_info: xmp_test_info = std::mem::uninitialized();
        let ret = xmp_test_module(CString::new(p.as_ref()).unwrap().as_ptr(), &mut test_info);
            (test_info, ret)
    };

    if ret != 0 {
        let int_kind = error::from_int_error_code(ret);
        return Err(error::XmpError::new(&format!("xmp_test_module call failed with code: {}", ret), int_kind));
    };

    let (t_name, t_type) = unsafe {
        let t_name = CStr::from_ptr(test_info.t_name.as_ptr());
        let t_name = t_name.to_owned().into_string().unwrap();
        let t_type = CStr::from_ptr(test_info.t_type.as_ptr());
        let t_type = t_type.to_owned().into_string().unwrap();

        (t_name, t_type)
    };

    return Ok(TestModuleInfo{ t_name, t_type });
}

