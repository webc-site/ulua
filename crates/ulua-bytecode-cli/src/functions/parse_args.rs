//! Source: `CLI/src/Bytecode.cpp:47-97`

use alloc::string::String;

use ulua_cli_lib::{
  functions::{
    parse_level_arg::apply_level_arg,
    report_missing_filename_option::report_missing_filename_option,
    report_unrecognized_option::report_unrecognized_option, set_luau_flags_flags::set_luau_flags,
  },
  records::global_options::{set_debug_level, set_optimization_level},
};

use crate::functions::display_help::display_help;

/// cpp `main` 里 `std::string summaryFile("bytecode-summary.json")` 的默认值
/// （`CLI/src/Bytecode.cpp:273`），也是 `--help` 文案里承诺的默认文件名。
pub(crate) const DEFAULT_SUMMARY_FILE: &str = "bytecode-summary.json";

/// cpp `parseArgs`（`CLI/src/Bytecode.cpp:47-97`）：cpp 用引用出参 + `bool`，
/// Rust 侧返回汇总文件名本身——成功是 `Some(name)`，参数非法是 `None`。
/// `argv0` 由 [`crate::functions::main::run`] 的共享前奏读出后传入。
pub(crate) fn parse_args(args: &[String], argv0: &str) -> Option<String> {
  let mut summary_file = String::from(DEFAULT_SUMMARY_FILE);

  for arg in args.iter().skip(1).map(String::as_str) {
    if arg == "-h" || arg == "--help" {
      // cpp displayHelp 内部 exit(0)
      display_help(argv0);
    } else if let Some(level_str) = arg.strip_prefix("-O") {
      // atoi 镜像 cpp: 前缀数字有效, 非数字得 0
      if !apply_level_arg(level_str, 0, 2, "Optimization", set_optimization_level) {
        return None;
      }
    } else if let Some(level_str) = arg.strip_prefix("-g") {
      if !apply_level_arg(level_str, 0, 2, "Debug", set_debug_level) {
        return None;
      }
    } else if let Some(filename) = arg.strip_prefix("--summary-file=") {
      if filename.is_empty() {
        report_missing_filename_option("--summary-file");
        return None;
      }
      summary_file = String::from(filename);
    } else if let Some(value) = arg.strip_prefix("--fflags=") {
      set_luau_flags(value);
    } else if arg.starts_with('-') {
      // cpp displayHelp 内部 exit(0)
      report_unrecognized_option(arg);
      display_help(argv0);
    }
  }

  Some(summary_file)
}
