//! cpp `serializeCompileStats` / `serializeLoweringStats` / `serializeBlockLinearizationStats` /
//! `serializeFunctionStats` (CLI/src/Compile.cpp:181-286) 的纯 Rust 改写:
//! `FILE* + fprintf` → `std::io::Write`; 键名保持上游驼峰 (JSON 输出契约)。
use std::io::{self, Write};

use ulua_code_gen::records::{
  block_linearization_stats::BlockLinearizationStats, function_stats::FunctionStats,
  lowering_stats::LoweringStats,
};

use crate::{
  macros::{write_name::WRITE_NAME, write_pair::WRITE_PAIR},
  records::compile_stats::CompileStats,
};

/// printf `%f` 默认 6 位定点小数; cpp `serializeFunctionStats`
pub fn serialize_function_stats<W: Write>(out: &mut W, stats: &FunctionStats) -> io::Result<()> {
  writeln!(out, "                {{")?;
  writeln!(out, "                    \"name\": \"{}\",", stats.name)?;
  WRITE_PAIR!(out, stats, "                    ", "line", line, "{},\n");
  WRITE_PAIR!(
    out,
    stats,
    "                    ",
    "bcodeCount",
    bcode_count,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "                    ",
    "irCount",
    ir_count,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "                    ",
    "asmCount",
    asm_count,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "                    ",
    "asmSize",
    asm_size,
    "{},\n"
  );
  write!(out, "                    \"bytecodeSummary\": ")?;

  let nesting_limit = stats.bytecode_summary.len();

  if nesting_limit == 0 {
    write!(out, "[]")?;
  } else {
    writeln!(out, "[")?;
    for (i, counts) in stats.bytecode_summary.iter().enumerate() {
      write!(out, "                        [")?;
      for (j, count) in counts.iter().enumerate() {
        write!(out, "{count}")?;
        if j < counts.len() - 1 {
          write!(out, ", ")?;
        }
      }
      write!(out, "]")?;
      if i < nesting_limit - 1 {
        writeln!(out, ",")?;
      }
    }
    write!(out, "\n                    ]")?;
  }

  write!(out, "\n                }}")
}

/// cpp `serializeBlockLinearizationStats`
fn serialize_block_linearization_stats<W: Write>(
  out: &mut W,
  stats: &BlockLinearizationStats,
) -> io::Result<()> {
  writeln!(out, "{{")?;

  WRITE_PAIR!(
    out,
    stats,
    "                ",
    "constPropInstructionCount",
    const_prop_instruction_count,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "                ",
    "timeSeconds",
    time_seconds,
    // cpp `WRITE_PAIR(..., "%f\n")`（Compile.cpp:224）：数值后换行，闭合 `}` 另起一行
    "{:.6}\n"
  );

  write!(out, "            }}")
}

/// cpp `serializeLoweringStats`
pub fn serialize_lowering_stats<W: Write>(out: &mut W, stats: &LoweringStats) -> io::Result<()> {
  writeln!(out, "{{")?;

  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "totalFunctions",
    total_functions,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "skippedFunctions",
    skipped_functions,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "spillsToSlot",
    spills_to_slot,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "spillsToRestore",
    spills_to_restore,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "maxSpillSlotsUsed",
    max_spill_slots_used,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "blocksPreOpt",
    blocks_pre_opt,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "blocksPostOpt",
    blocks_post_opt,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "maxBlockInstructions",
    max_block_instructions,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "regAllocErrors",
    reg_alloc_errors,
    "{},\n"
  );
  WRITE_PAIR!(
    out,
    stats,
    "            ",
    "loweringErrors",
    lowering_errors,
    "{},\n"
  );

  WRITE_NAME!(out, "            ", "blockLinearizationStats");
  serialize_block_linearization_stats(out, &stats.block_linearization_stats)?;
  writeln!(out, ",")?;

  WRITE_NAME!(out, "            ", "functions");
  let function_count = stats.functions.len();

  if function_count == 0 {
    write!(out, "[]")?;
  } else {
    writeln!(out, "[")?;
    for (i, function) in stats.functions.iter().enumerate() {
      serialize_function_stats(out, function)?;
      if i < function_count - 1 {
        writeln!(out, ",")?;
      }
    }
    write!(out, "\n            ]")?;
  }

  write!(out, "\n        }}")
}

/// cpp `serializeCompileStats`
pub fn serialize_compile_stats<W: Write>(out: &mut W, stats: &CompileStats) -> io::Result<()> {
  writeln!(out, "{{")?;

  WRITE_PAIR!(out, stats, "        ", "lines", lines, "{},\n");
  WRITE_PAIR!(out, stats, "        ", "bytecode", bytecode, "{},\n");
  WRITE_PAIR!(
    out,
    stats,
    "        ",
    "bytecodeInstructionCount",
    bytecode_instruction_count,
    "{},\n"
  );
  WRITE_PAIR!(out, stats, "        ", "codegen", codegen, "{},\n");
  WRITE_PAIR!(out, stats, "        ", "readTime", read_time, "{:.6}");
  writeln!(out, ",")?;
  WRITE_PAIR!(out, stats, "        ", "miscTime", misc_time, "{:.6}");
  writeln!(out, ",")?;
  WRITE_PAIR!(out, stats, "        ", "parseTime", parse_time, "{:.6}");
  writeln!(out, ",")?;
  WRITE_PAIR!(out, stats, "        ", "compileTime", compile_time, "{:.6}");
  writeln!(out, ",")?;
  WRITE_PAIR!(out, stats, "        ", "codegenTime", codegen_time, "{:.6}");
  writeln!(out, ",")?;

  WRITE_NAME!(out, "        ", "lowerStats");
  serialize_lowering_stats(out, &stats.lower_stats)?;

  write!(out, "\n    }}")
}
