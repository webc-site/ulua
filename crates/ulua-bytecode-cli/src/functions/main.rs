//! Source: `CLI/src/Bytecode.cpp:269-299` (`main`)
use alloc::{string::String, vec::Vec};

use ulua_cli_lib::functions::{
  assertion_handler::install_assertion_handler, get_source_files::get_source_files_from_slice,
  set_luau_flags_default::set_luau_flags_default,
};
use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::{
  analyze_file::analyze_file, parse_args::parse_args, serialize_summaries::serialize_summaries,
};

/// cpp `int main(int argc, char** argv)` (`CLI/src/Bytecode.cpp:269-299`)
pub fn run(args: &[String]) -> i32 {
  // Luau::assertHandler() = assertionHandler;
  install_assertion_handler();

  set_luau_flags_default();

  // cpp: unsigned nestingLimit = 0;
  const NESTING_LIMIT: u32 = 0;

  let Some(summary_file) = parse_args(args) else {
    return 1;
  };

  let files = get_source_files_from_slice(args);

  // cpp `scriptSummaries[i]` 在未 resize 的空 vector 上索引是上游 UB; 此处以 push 等价实现
  let mut script_summaries: Vec<Vec<FunctionBytecodeSummary>> = Vec::with_capacity(files.len());

  for file in &files {
    let Some(script_summary) = analyze_file(file, NESTING_LIMIT) else {
      return 1;
    };

    script_summaries.push(script_summary);
  }

  if !serialize_summaries(&files, &script_summaries, &summary_file) {
    return 1;
  }

  println!("Bytecode summary written to '{summary_file}'");

  0
}
