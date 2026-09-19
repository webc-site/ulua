use std::io::{Result, Write};

use ulua_cli_lib::functions::{
  escape_filename::escape_filename, write_json_entries::write_json_entries,
};
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

  write_json_entries(out, script_summary, |out, summary| {
    serialize_function_summary(summary, out)
  })?;

  write!(out, "    ]")?;

  Ok(())
}
