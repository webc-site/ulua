use alloc::{string::String, vec::Vec};
use core::ffi::c_char;

use ulua_cli_lib::functions::{
  get_source_files::get_source_files, set_luau_flags_default::set_luau_flags_default,
};
use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;
use ulua_common::functions::assert_handler::assert_handler;

use crate::functions::{
  analyze_file::analyze_file, assertion_handler::assertion_handler, parse_args::parse_args,
  serialize_summaries::serialize_summaries,
};

unsafe extern "C-unwind" fn assertion_handler_adapter(
  expr: *const c_char,
  file: *const c_char,
  line: i32,
  function: *const c_char,
) -> i32 {
  unsafe { assertion_handler(expr, file, line, function) }
}

/// # Safety
///
/// `argv` 必须是指向长度至少为 `argc` 的有效 `*mut c_char` 数组指针，且各指针指向合法的以 NUL 结尾的 C 字符串。
pub unsafe fn main(argc: i32, argv: *mut *mut c_char) -> i32 {
  *assert_handler() = Some(assertion_handler_adapter);

  set_luau_flags_default();

  let mut summary_file = String::from("bytecode-summary.json");
  let nesting_limit = 0;

  if !parse_args(argc, argv, &mut summary_file) {
    return 1;
  }

  let files = unsafe { get_source_files(argc, argv) };
  let file_count = files.len();

  let mut script_summaries: Vec<Vec<FunctionBytecodeSummary>> = Vec::with_capacity(file_count);

  for file in &files {
    let mut script_summary = Vec::new();

    if !analyze_file(file, nesting_limit, &mut script_summary) {
      return 1;
    }

    script_summaries.push(script_summary);
  }

  if !serialize_summaries(&files, &script_summaries, &summary_file) {
    return 1;
  }

  println!("Bytecode summary written to '{}'", summary_file);

  0
}
