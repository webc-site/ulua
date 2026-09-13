use std::io::{Result, Write};

use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::{
  escape_filename::escape_filename, serialize_function_summary::serialize_function_summary,
};

/// 序列化单个脚本内所有函数的字节码统计摘要
pub fn serialize_script_summary<W: Write>(
  file: &str,
  script_summary: &[FunctionBytecodeSummary],
  out: &mut W,
) -> Result<()> {
  let clean_file = file.trim_matches('\0');
  let escaped = escape_filename(clean_file);
  let function_count = script_summary.len();

  writeln!(out, "    \"{escaped}\": [")?;

  for (i, summary) in script_summary.iter().enumerate() {
    serialize_function_summary(summary, out)?;
    if i == function_count - 1 {
      writeln!(out)?;
    } else {
      writeln!(out, ",")?;
    }
  }

  write!(out, "    ]")?;

  Ok(())
}
