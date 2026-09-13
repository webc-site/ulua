//! Node: `cxx:Function:Luau.Bytecode.CLI:CLI/src/Bytecode.cpp:47:parse_args`
//! Source: `CLI/src/Bytecode.cpp:47-97`

use alloc::string::String;
use core::ffi::c_char;
use std::ffi::CStr;

use ulua_cli_lib::functions::set_luau_flags_flags_alt_b::set_luau_flags_c_char;

use crate::{
  functions::display_help::display_help,
  records::global_options::{set_debug_level, set_optimization_level},
};

pub(crate) fn parse_args(argc: i32, argv: *mut *mut c_char, summary_file: &mut String) -> bool {
  for i in 1..argc {
    let arg_ptr = unsafe { *argv.add(i as usize) };
    let arg = unsafe { CStr::from_ptr(arg_ptr) };
    let arg_str = arg.to_str().unwrap();

    if arg_str == "-h" || arg_str == "--help" {
      let argv0 = unsafe { CStr::from_ptr(*argv) };
      display_help(argv0.to_str().unwrap());
    } else if let Some(level_str) = arg_str.strip_prefix("-O") {
      let level: i32 = level_str.parse().unwrap_or(0);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Optimization level must be between 0 and 2 inclusive.");
        return false;
      }
      set_optimization_level(level);
    } else if let Some(level_str) = arg_str.strip_prefix("-g") {
      let level: i32 = level_str.parse().unwrap_or(0);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Debug level must be between 0 and 2 inclusive.");
        return false;
      }
      set_debug_level(level);
    } else if let Some(filename) = arg_str.strip_prefix("--summary-file=") {
      if filename.is_empty() {
        eprintln!("Error: filename missing for '--summary-file'.\n");
        return false;
      }
      *summary_file = filename.to_string();
    } else if arg_str.starts_with("--fflags=") {
      // setLuauFlags(argv[i] + 9);
      unsafe {
        set_luau_flags_c_char(arg_ptr.add(9) as *const c_char);
      }
    } else if arg_str.starts_with('-') {
      eprintln!("Error: Unrecognized option '{}'.\n", arg_str);
      let argv0 = unsafe { CStr::from_ptr(*argv) };
      display_help(argv0.to_str().unwrap());
    }
  }

  true
}
