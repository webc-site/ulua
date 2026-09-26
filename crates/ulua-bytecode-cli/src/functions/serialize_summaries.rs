use std::{
  fs::File,
  io::{BufWriter, Result, Write},
};

use ulua_cli_lib::functions::write_json_entries::write_json_entries;
use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::serialize_script_summary::serialize_script_summary;

/// cpp `serializeSummaries` (CLI/src/Bytecode.cpp:235-267): 把所有文件的摘要写成
/// 一份 JSON 映射 `{"<file>": [ ... ], ...}` 落到 `summary_file`。
///
/// cpp 只对 `fopen` 失败返回 false（`fprintf`/`fclose` 的返回值被忽略）；Rust 侧把
/// 序列化/写盘失败也判为失败并报出真实原因，避免静默留下半截 JSON 却退出 0。
pub(crate) fn serialize_summaries(
  files: &[String],
  script_summaries: &[Vec<FunctionBytecodeSummary>],
  summary_file: &str,
) -> bool {
  // summary_file 直接来自 argv（`parse_args` 用 strip_prefix 取出的 &str 片段），
  // 与 cpp `summaryFile.c_str()` 一样不可能含 NUL，无需裁剪
  let Ok(file) = File::create(summary_file) else {
    eprintln!("Unable to open '{summary_file}'.");
    return false;
  };

  let mut writer = BufWriter::new(file);
  if let Err(error) = write_summaries(&mut writer, files, script_summaries) {
    eprintln!("Unable to write '{summary_file}': {error}");
    return false;
  }

  true
}

/// JSON 主体：条目分隔符统一走 `write_json_entries`
fn write_summaries<W: Write>(
  writer: &mut W,
  files: &[String],
  script_summaries: &[Vec<FunctionBytecodeSummary>],
) -> Result<()> {
  writeln!(writer, "{{")?;
  write_json_entries(
    writer,
    files.iter().zip(script_summaries.iter()),
    |writer, (path, summary)| serialize_script_summary(path, summary, writer),
  )?;
  write!(writer, "}}")?;

  // BufWriter 的 drop 会吞掉 flush 错误，必须显式 flush 才能发现问题
  writer.flush()
}
