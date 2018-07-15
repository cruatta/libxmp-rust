//lib.rs
use ffi::*;
use libc::{ free, c_void };

pub struct Context {
    pub xmp_context: *mut xmp_context
}


impl Context {
    pub fn new() -> Context {
        unsafe {
            let xmp_context = xmp_create_context();
            match xmp_context.as_ref() {
                Some(_) => Context{ xmp_context },
                None => panic!("Cannot allocate context")
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { free(self.xmp_context as *mut c_void) };
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_context() {
        Context::new();
        assert!(true);
    }

}

