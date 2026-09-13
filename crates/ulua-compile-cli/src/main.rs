//! `ulua-compile` — command-line Luau source-to-bytecode compiler (binary entry point).
//!
//! Thin wrapper over the library `main(argc, argv)` (faithful port of the upstream
//! `luau-compile` CLI in CLI/src/Compile.cpp). Marshals `std::env::args()` into a
//! NUL-terminated owned `argv` so the FileUtils/Flags ports (which take
//! `int argc, char** argv`) can be called faithfully.

use core::ffi::c_char;
use std::{env::args, ffi::CString, process::exit};

use ulua_compile_cli::run_main;

fn main() {
  let args: Vec<String> = args().collect();
  let mut c_args: Vec<CString> = args
    .iter()
    .map(|arg| CString::new(arg.as_str()).unwrap())
    .collect();
  let mut argv: Vec<*mut c_char> = c_args
    .iter_mut()
    .map(|arg| arg.as_ptr() as *mut c_char)
    .collect();

  let exit_code = unsafe { run_main(args.len() as i32, argv.as_mut_ptr()) };
  exit(exit_code);
}
