//lib.rs

use std::path::Path;
use std::ffi::{ CString, CStr };
use std::mem::*;

use error::*;
use ffi::*;

#[cfg(test)]
mod tests {

    use super::*;

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
    pub t_name: String,
    pub t_type: String
}


pub fn test_module(path: &Path) -> Result<TestModuleInfo, XmpError> {
    let p = path.to_string_lossy();
    let p = CString::new(p.as_ref()).unwrap();
    let p_ptr = p.as_ptr();

    let (test_info, ret) = unsafe {
        let mut test_info: xmp_test_info = uninitialized();
        let ret = xmp_test_module(p_ptr, &mut test_info);
        (test_info, ret)
    };

    if ret != 0 {
        let int_kind = from_int_error_code(ret);
        return Err(XmpError::new(&format!("xmp_test_module call failed with code: {}", ret), int_kind));
    };

    let t_name_ptr = test_info.t_name.as_ptr();
    let t_type_ptr = test_info.t_type.as_ptr();

    let (t_name, t_type) = unsafe {
        (CStr::from_ptr(t_name_ptr), CStr::from_ptr(t_type_ptr))
    };

    let t_name = t_name.to_owned().into_string().unwrap();
    let t_type = t_type.to_owned().into_string().unwrap();

    return Ok(TestModuleInfo{ t_name, t_type });
}
