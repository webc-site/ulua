use std::io::{Result, Write};

use ulua_cli_lib::functions::escape_filename::escape_filename;
use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::serialize_function_summary::serialize_function_summary;

/// 序列化单个脚本内所有函数的字节码统计摘要
pub fn serialize_script_summary<W: Write>(
  file: &str,
  script_summary: &[FunctionBytecodeSummary],
  out: &mut W,
) -> Result<()> {
  let clean_file = file.trim_matches('\0');
  let escaped = escape_filename(clean_file);

  writeln!(out, "    \"{escaped}\": [")?;

  // 分隔符用 peekable 前瞻，免去下标比较
  let mut entries = script_summary.iter().peekable();
  while let Some(summary) = entries.next() {
    serialize_function_summary(summary, out)?;
    if entries.peek().is_some() {
      writeln!(out, ",")?;
    } else {
      writeln!(out)?;
    }
  }

  write!(out, "    ]")?;

  Ok(())
}
