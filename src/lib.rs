extern crate libc;

use std::path::Path;
use std::ffi::{ CString, CStr };
use std::mem::uninitialized;

mod ffi;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_test_module_xm() {
        let path = Path::new("./test/lol.xm");
        let ret = test_module(&path);
        assert_eq!(ret, "playboy");
    }
}

pub fn test_module(path: &Path) -> String {
    use ffi::*;

    if let Some(p) = path.to_str() {

        let name: CString = unsafe {
            let mut module: xmp_test_info = uninitialized();
            let ret = xmp_test_module(CString::new(p).unwrap().as_ptr(), &mut module);
            if ret != 0 {
                panic!("wtf");
            }
            let name = CStr::from_ptr(module.module_name.as_ptr());
            name.to_owned()
        };

        let name = name.into_string().unwrap();
        println!("{:?}", path);
        return name;
    }
    return String::from("fuck off");

}

