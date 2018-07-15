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

