//lib.rs

extern crate libc;

mod error;
mod ffi;
pub mod context;
pub mod test_module;
pub mod load_module;

use std::path::Path;
use load_module::*;
use context::Context;



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lib() {
        use ffi::*;

        let note_names = vec![
	          "C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "
        ];

        let path = Path::new("./test/test0.xm");
        let context = Context::new();

        match load_module(&context, &path) {
            Ok(_) => {
                println!("no working")
            },
            Err(y) => panic!(y.details)
        }

        unsafe {
            xmp_start_player(context.xmp_context, 44100, 0);

            let mut module_info: xmp_module_info = std::mem::uninitialized();
            xmp_get_module_info(context.xmp_context, module_info);
            let module = module_info.module;
            println!("Name: {}", (*module).m_name);

            xmp_end_player(context.xmp_context);
            xmp_release_module(context.xmp_context);
        }
        assert!(false);
    }
}
