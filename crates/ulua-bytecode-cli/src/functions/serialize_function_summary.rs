use std::io::{Result, Write};

use ulua_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

/// 序列化单个函数的字节码统计摘要为 JSON 对象
pub fn serialize_function_summary<W: Write>(
  summary: &FunctionBytecodeSummary,
  out: &mut W,
) -> Result<()> {
  let nesting_limit = summary.get_nesting_limit();
  let op_limit = summary.get_op_limit();

  // summary 的 source/name 由 `CStr::from_ptr` 生成（不含终止符），无需再去 NUL
  let source = summary.get_source();
  let name = summary.get_name();

  writeln!(out, "        {{")?;
  writeln!(out, "            \"source\": \"{source}\",")?;
  writeln!(out, "            \"name\": \"{name}\",")?;
  writeln!(out, "            \"line\": {},", summary.get_line())?;
  writeln!(out, "            \"nestingLimit\": {nesting_limit},")?;
  write!(out, "            \"counts\": [")?;

  for nesting in 0..=nesting_limit {
    write!(out, "\n                [")?;

    // 单次遍历输出 count 序列，首个元素前置，其余自带分隔符
    let mut counts = (0..op_limit).map(|op| summary.get_count(nesting, op as u8));
    if let Some(first) = counts.next() {
      write!(out, "{first}")?;
      counts.try_for_each(|count| write!(out, ", {count}"))?;
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
