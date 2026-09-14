use std::io::{Result, Write};

use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

/// 序列化单个函数的字节码统计摘要为 JSON 对象
pub fn serialize_function_summary<W: Write>(
  summary: &FunctionBytecodeSummary,
  out: &mut W,
) -> Result<()> {
  let nesting_limit = summary.get_nesting_limit();
  let op_limit = summary.get_op_limit();

  let source = summary.get_source().trim_matches('\0');
  let name = summary.get_name().trim_matches('\0');

  writeln!(out, "        {{")?;
  writeln!(out, "            \"source\": \"{source}\",")?;
  writeln!(out, "            \"name\": \"{name}\",")?;
  writeln!(out, "            \"line\": {},", summary.get_line())?;
  writeln!(out, "            \"nestingLimit\": {nesting_limit},")?;
  write!(out, "            \"counts\": [")?;

  for nesting in 0..=nesting_limit {
    write!(out, "\n                [")?;

    for i in 0..op_limit {
      let count = summary.get_count(nesting, i as u8);
      write!(out, "{count}")?;

      if i < op_limit - 1 {
        write!(out, ", ")?;
      }
    }

    write!(out, "]")?;

    if nesting < nesting_limit {
      write!(out, ",")?;
    }
  }

  writeln!(out, "\n            ]")?;
  write!(out, "        }}")?;

  Ok(())
}
