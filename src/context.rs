//lib.rs
use ffi::*;

pub struct Context {
    pub(crate) state: xmp_context
}


impl Context {
    pub fn new() -> Context {
        unsafe {
            let xmp_context = xmp_create_context();
            match xmp_context.as_ref() {
                Some(_) => Context{ state: xmp_context },
                None => panic!("Cannot allocate context")
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { xmp_free_context(self.state) };
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

