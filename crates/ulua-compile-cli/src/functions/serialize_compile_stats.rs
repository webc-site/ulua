//! cpp `serializeCompileStats` / `serializeLoweringStats` / `serializeBlockLinearizationStats` /
//! `serializeFunctionStats` (CLI/src/Compile.cpp:181-286) 的纯 Rust 改写:
//! `FILE* + fprintf` → `std::io::Write`; 键名保持上游驼峰 (JSON 输出契约)。
use std::io::{self, Write};

use ulua_cli_lib::functions::write_json_entries::write_json_entries;
use ulua_code_gen::records::{
  block_linearization_stats::BlockLinearizationStats, function_stats::FunctionStats,
  lowering_stats::LoweringStats,
};

use crate::{
  macros::{write_name::WRITE_NAME, write_pair::WRITE_PAIR},
  records::compile_stats::CompileStats,
};

/// 数字平铺的 `, ` 连接（无换行）：`write_json_entries` 的条目分隔是 `,\n`，
/// 这里是 `bytecodeSummary` 内层数组用的纯 `, `，故单列此助手。
fn write_counts<W: Write>(out: &mut W, counts: &[u32]) -> io::Result<()> {
  let mut first = true;
  for count in counts {
    if !first {
      write!(out, ", ")?;
    }
    first = false;
    write!(out, "{count}")?;
  }
  Ok(())
}

/// printf `%f` 默认 6 位定点小数; cpp `serializeFunctionStats`
pub(crate) fn serialize_function_stats<W: Write>(
  out: &mut W,
  stats: &FunctionStats,
) -> io::Result<()> {
  // 本函数固定缩进的键值对快捷形式（缩进烘焙进展开体）
  macro_rules! pair {
    ($key:literal, $field:ident) => {
      WRITE_PAIR!(out, stats, "                    ", $key, $field)
    };
  }

  writeln!(out, "                {{")?;
  writeln!(out, "                    \"name\": \"{}\",", stats.name)?;
  pair!("line", line);
  pair!("bcodeCount", bcode_count);
  pair!("irCount", ir_count);
  pair!("asmCount", asm_count);
  pair!("asmSize", asm_size);
  write!(out, "                    \"bytecodeSummary\": ")?;

  if stats.bytecode_summary.is_empty() {
    write!(out, "[]")?;
  } else {
    writeln!(out, "[")?;
    write_json_entries(out, &stats.bytecode_summary, |out, counts| {
      write!(out, "                        [")?;
      write_counts(out, counts)?;
      write!(out, "]")
    })?;
    write!(out, "                    ]")?;
  }

  write!(out, "\n                }}")
}

/// cpp `serializeBlockLinearizationStats`
fn serialize_block_linearization_stats<W: Write>(
  out: &mut W,
  stats: &BlockLinearizationStats,
) -> io::Result<()> {
  // 本函数固定缩进的键值对快捷形式；自定义格式（printf 语义）走三参臂
  macro_rules! pair {
    ($key:literal, $field:ident) => {
      WRITE_PAIR!(out, stats, "                ", $key, $field)
    };
    ($key:literal, $field:ident, $format:literal) => {
      WRITE_PAIR!(out, stats, "                ", $key, $field, $format)
    };
  }

  writeln!(out, "{{")?;

  pair!("constPropInstructionCount", const_prop_instruction_count);
  // cpp `WRITE_PAIR(..., "%f\n")`（Compile.cpp:224）：数值后换行，闭合 `}` 另起一行
  pair!("timeSeconds", time_seconds, "{:.6}\n");

  write!(out, "            }}")
}

/// cpp `serializeLoweringStats`
pub(crate) fn serialize_lowering_stats<W: Write>(
  out: &mut W,
  stats: &LoweringStats,
) -> io::Result<()> {
  // 本函数固定缩进的键值对快捷形式
  macro_rules! pair {
    ($key:literal, $field:ident) => {
      WRITE_PAIR!(out, stats, "            ", $key, $field)
    };
  }

  writeln!(out, "{{")?;

  pair!("totalFunctions", total_functions);
  pair!("skippedFunctions", skipped_functions);
  pair!("spillsToSlot", spills_to_slot);
  pair!("spillsToRestore", spills_to_restore);
  pair!("maxSpillSlotsUsed", max_spill_slots_used);
  pair!("blocksPreOpt", blocks_pre_opt);
  pair!("blocksPostOpt", blocks_post_opt);
  pair!("maxBlockInstructions", max_block_instructions);
  pair!("regAllocErrors", reg_alloc_errors);
  pair!("loweringErrors", lowering_errors);

  WRITE_NAME!(out, "            ", "blockLinearizationStats");
  serialize_block_linearization_stats(out, &stats.block_linearization_stats)?;
  writeln!(out, ",")?;

  WRITE_NAME!(out, "            ", "functions");

  if stats.functions.is_empty() {
    write!(out, "[]")?;
  } else {
    writeln!(out, "[")?;
    write_json_entries(out, &stats.functions, |out, function| {
      serialize_function_stats(out, function)
    })?;
    write!(out, "            ]")?;
  }

  write!(out, "\n        }}")
}

/// cpp `serializeCompileStats`
pub(crate) fn serialize_compile_stats<W: Write>(
  out: &mut W,
  stats: &CompileStats,
) -> io::Result<()> {
  // 本函数固定缩进的键值对快捷形式；自定义格式（printf 语义）走三参臂
  macro_rules! pair {
    ($key:literal, $field:ident) => {
      WRITE_PAIR!(out, stats, "        ", $key, $field)
    };
    ($key:literal, $field:ident, $format:literal) => {
      WRITE_PAIR!(out, stats, "        ", $key, $field, $format)
    };
  }

  writeln!(out, "{{")?;

  pair!("lines", lines);
  pair!("bytecode", bytecode);
  pair!("bytecodeInstructionCount", bytecode_instruction_count);
  pair!("codegen", codegen);
  // printf `%f` 默认 6 位定点小数：`"{:.6},\n"` 即原 `WRITE_PAIR(..., "{:.6}")` + `writeln!(",")` 的合并
  pair!("readTime", read_time, "{:.6},\n");
  pair!("miscTime", misc_time, "{:.6},\n");
  pair!("parseTime", parse_time, "{:.6},\n");
  pair!("compileTime", compile_time, "{:.6},\n");
  pair!("codegenTime", codegen_time, "{:.6},\n");

  WRITE_NAME!(out, "        ", "lowerStats");
  serialize_lowering_stats(out, &stats.lower_stats)?;

  write!(out, "\n    }}")
}
