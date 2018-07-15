//lib.rs
use ffi::*;

pub struct Context {
    pub xmp_context: Box<xmp_context>
}


impl Context {
    pub fn new() -> Context {
        unsafe {
            let xmp_context = Box::from_raw(xmp_create_context());
            Context { xmp_context }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_context() {
        let x = Context::new();
        //assert!(true, false);
    }

}

