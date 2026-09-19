//! Source: `CLI/src/Bytecode.cpp:47-97`

use alloc::string::String;

use ulua_cli_lib::functions::{
  parse_level_arg::parse_level_arg, set_luau_flags_flags_alt_b::set_luau_flags,
};

use crate::{
  functions::display_help::display_help,
  records::global_options::{set_debug_level, set_optimization_level},
};

pub(crate) fn parse_args(args: &[String], summary_file: &mut String) -> bool {
  let argv0 = args.first().map(String::as_str).unwrap_or("ulua-bytecode");

  for arg in args.iter().skip(1).map(String::as_str) {
    if arg == "-h" || arg == "--help" {
      // cpp displayHelp 内部 exit(0)
      display_help(argv0);
    } else if let Some(level_str) = arg.strip_prefix("-O") {
      // atoi 镜像 cpp: 前缀数字有效, 非数字得 0
      match parse_level_arg(level_str, 0, 2, "Optimization") {
        Some(level) => set_optimization_level(level),
        None => return false,
      }
    } else if let Some(level_str) = arg.strip_prefix("-g") {
      match parse_level_arg(level_str, 0, 2, "Debug") {
        Some(level) => set_debug_level(level),
        None => return false,
      }
    } else if let Some(filename) = arg.strip_prefix("--summary-file=") {
      if filename.is_empty() {
        // cpp 消息以 "\n\n" 结尾 (eprintln! 自带一个换行)
        eprintln!("Error: filename missing for '--summary-file'.\n");
        return false;
      }
      *summary_file = filename.to_string();
    } else if let Some(value) = arg.strip_prefix("--fflags=") {
      set_luau_flags(value);
    } else if arg.starts_with('-') {
      // cpp 消息以 "\n\n" 结尾; displayHelp 内部 exit(0)
      eprintln!("Error: Unrecognized option '{arg}'.\n");
      display_help(argv0);
    }
  }

  true
}
