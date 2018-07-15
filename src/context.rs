//lib.rs
use ffi::*;

pub struct Context {
    pub xmp_context: *mut xmp_context
}


impl Context {
    pub fn new() -> Context {
        unsafe {
            let context: *mut xmp_context = xmp_create_context();
            match context.as_ref() {
                Some(_) => Context{ xmp_context: context },
                None => panic!("Cannot allocate context")
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_context() {
        let x = Context::new();
    }

}

