//! Source: `CLI/src/Bytecode.cpp:269-299` (`main`)
use alloc::{string::String, vec::Vec};

use ulua_cli_lib::functions::{
  cli_preamble::cli_preamble, get_source_files::get_source_files_from_slice,
};
use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::{
  analyze_file::analyze_file, parse_args::parse_args, serialize_summaries::serialize_summaries,
};

/// cpp `int main(int argc, char** argv)` (`CLI/src/Bytecode.cpp:269-299`)
pub fn run(args: &[String]) -> i32 {
  // 断言处理器 + 默认旗标 + argv0 收敛于共享前奏；`--help`/`-h` 由 parse_args
  // 逐参循环识别（cpp 语义为全参扫描），故丢弃前奏的顶层早退标记。
  let argv0 = cli_preamble(args, "ulua-bytecode").argv0;

  // cpp: unsigned nestingLimit = 0;
  const NESTING_LIMIT: u32 = 0;

  let Some(summary_file) = parse_args(args, argv0) else {
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
