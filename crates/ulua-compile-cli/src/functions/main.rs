//! Source: `CLI/src/Compile.cpp:487-740` (`main`)
use alloc::{ffi::CString, string::String, vec::Vec};
use std::{
  fs::File,
  io::{BufWriter, Result, Write},
};

use ulua_cli_lib::functions::{
  assertion_handler::install_assertion_handler, escape_filename::escape_filename,
  get_source_files::get_source_files_from_slice, parse_level_arg::parse_level_arg,
  set_luau_flags_default::set_luau_flags_default, set_luau_flags_flags_alt_b::set_luau_flags,
  time_trace_unsupported::time_trace_unsupported,
};
use ulua_code_gen::{
  enums::{function_stats_flags::FunctionStatsFlags, target::Target},
  records::lowering_stats::FUNCTION_STATS_ENABLE,
};
use ulua_common::FFlag::DebugLuauTimeTracing;

use crate::{
  enums::{compile_format::CompileFormat, record_stats::RecordStats},
  functions::{
    compile_file::compile_file, display_help::display_help, get_compile_format::get_compile_format,
    serialize_compile_stats::serialize_compile_stats,
  },
  records::{compile_stats::CompileStats, global_options::mutate},
};

/// cpp 输出 `stats.lines / 1000` (KLOC)
const LINES_PER_KLOC: usize = 1000;
/// cpp 输出 `stats.bytecode / 1024` (KB)
const BYTES_PER_KB: usize = 1024;

/// cpp `int main(int argc, char** argv)` (CLI/src/Compile.cpp:487-740)
pub fn run(args: &[String]) -> i32 {
  // Luau::assertHandler() = assertionHandler;
  install_assertion_handler();

  set_luau_flags_default();

  let mut compile_format = CompileFormat::Text;
  let mut assembly_target = Target::Host;
  let mut record_stats = RecordStats::None;
  let mut stats_file = String::from("stats.json");
  let mut bytecode_summary = false;
  let mut dump_constants = false;

  let argv0 = args.first().map(String::as_str).unwrap_or("ulua-compile");

  for arg in args.iter().skip(1).map(String::as_str) {
    if arg == "-h" || arg == "--help" {
      display_help(argv0);
      return 0;
    } else if let Some(suffix) = arg.strip_prefix("-O") {
      match parse_level_arg(suffix, 0, 2, "Optimization") {
        Some(level) => unsafe {
          mutate(|opts| opts.optimization_level = level);
        },
        None => return 1,
      }
    } else if let Some(suffix) = arg.strip_prefix("-g") {
      match parse_level_arg(suffix, 0, 2, "Debug") {
        Some(level) => unsafe {
          mutate(|opts| opts.debug_level = level);
        },
        None => return 1,
      }
    } else if let Some(suffix) = arg.strip_prefix("-t") {
      match parse_level_arg(suffix, 0, 1, "Type info") {
        Some(level) => unsafe {
          mutate(|opts| opts.type_info_level = level);
        },
        None => return 1,
      }
    } else if let Some(value) = arg.strip_prefix("--target=") {
      let target = match value {
        "a64" => Target::A64,
        "a64_nf" => Target::A64NoFeatures,
        "x64" => Target::X64SystemV,
        "x64_ms" => Target::X64Windows,
        _ => {
          eprintln!("Error: unknown target");
          return 1;
        }
      };
      assembly_target = target;
    } else if arg == "--timetrace" {
      DebugLuauTimeTracing.set(true);
    } else if let Some(value) = arg.strip_prefix("--record-stats=") {
      record_stats = match value {
        "total" => RecordStats::Total,
        "file" => RecordStats::File,
        "function" => RecordStats::Function,
        _ => {
          eprintln!("Error: unknown 'granularity' for '--record-stats'.");
          return 1;
        }
      };
    } else if arg.starts_with("--bytecode-summary") {
      bytecode_summary = true;
    } else if arg == "--dump-constants" {
      dump_constants = true;
    } else if arg == "--dump-regspills" {
      // cpp 记入 globalOptions.dumpRegSpills; ulua-code-gen 尚无 include_reg_spills
      // 字段 (跨 crate 缺口), 此处仅保持参数解析行为一致
      unsafe {
        mutate(|opts| opts.dump_reg_spills = true);
      }
    } else if let Some(value) = arg.strip_prefix("--stats-file=") {
      stats_file = String::from(value);

      if stats_file.is_empty() {
        // cpp 消息以 "\n\n" 结尾 (eprintln! 自带一个换行)
        eprintln!("Error: filename missing for '--stats-file'.\n");
        return 1;
      }
    } else if let Some(value) = arg.strip_prefix("--fflags=") {
      set_luau_flags(value);
    } else if let Some(value) = arg.strip_prefix("--vector-lib=") {
      unsafe {
        mutate(|opts| opts.vector_lib = Some(CString::new(value).unwrap_or_default()));
      }
    } else if let Some(value) = arg.strip_prefix("--vector-ctor=") {
      unsafe {
        mutate(|opts| opts.vector_ctor = Some(CString::new(value).unwrap_or_default()));
      }
    } else if let Some(value) = arg.strip_prefix("--vector-type=") {
      unsafe {
        mutate(|opts| opts.vector_type = Some(CString::new(value).unwrap_or_default()));
      }
    } else if arg.starts_with("--parse-cst") {
      unsafe {
        mutate(|opts| opts.parse_cst = true);
      }
    } else if arg.starts_with("--only-parse") {
      unsafe {
        mutate(|opts| opts.only_parse = true);
      }
    } else if let Some(format_name) = arg.strip_prefix("--") {
      if let Some(format) = get_compile_format(format_name) {
        compile_format = format;
      } else {
        // cpp 消息以 "\n\n" 结尾
        eprintln!("Error: Unrecognized option '{arg}'.\n");
        display_help(argv0);
        return 1;
      }
    } else if arg.starts_with('-') {
      // cpp 消息以 "\n\n" 结尾
      eprintln!("Error: Unrecognized option '{arg}'.\n");
      display_help(argv0);
      return 1;
    }
  }

  if bytecode_summary && record_stats != RecordStats::Function {
    eprintln!("'Error: Required '--record-stats=function' for '--bytecode-summary'.");
    return 1;
  }

  // cpp: LUAU_ENABLE_TIME_TRACE 未定义时的守卫
  if time_trace_unsupported() {
    return 1;
  }

  let files = get_source_files_from_slice(args);
  let mut stats = CompileStats::default();

  let stats_by_file = record_stats == RecordStats::File || record_stats == RecordStats::Function;
  let mut file_stats: Vec<CompileStats> = Vec::new();
  if stats_by_file {
    file_stats.reserve(files.len());
  }

  let mut failed = 0usize;
  let function_stats = (if record_stats == RecordStats::Function {
    FUNCTION_STATS_ENABLE
  } else {
    0
  }) | if bytecode_summary {
    FunctionStatsFlags::FunctionStatsBytecodeSummary as u32
  } else {
    0
  };

  for path in &files {
    let mut file_stat = CompileStats::default();
    file_stat.lower_stats.function_stats_flags = function_stats;

    if !compile_file(
      path,
      compile_format,
      assembly_target,
      &mut file_stat,
      dump_constants,
    ) {
      failed += 1;
    }

    stats += &file_stat;

    if stats_by_file {
      file_stats.push(file_stat);
    }
  }

  if compile_format == CompileFormat::Null {
    println!(
      "Compiled {} KLOC into {} KB bytecode (read {:.2}s, parse {:.2}s, compile {:.2}s)",
      stats.lines / LINES_PER_KLOC,
      stats.bytecode / BYTES_PER_KB,
      stats.read_time,
      stats.parse_time,
      stats.compile_time
    );
  } else if compile_format == CompileFormat::CodegenNull {
    println!(
      "Compiled {} KLOC into {} KB bytecode => {} KB native code ({:.2}x) (read {:.2}s, parse {:.2}s, compile {:.2}s, codegen {:.2}s)",
      stats.lines / LINES_PER_KLOC,
      stats.bytecode / BYTES_PER_KB,
      stats.codegen / BYTES_PER_KB,
      if stats.bytecode == 0 {
        0.0
      } else {
        stats.codegen as f64 / stats.bytecode as f64
      },
      stats.read_time,
      stats.parse_time,
      stats.compile_time,
      stats.codegen_time
    );

    println!(
      "Lowering: regalloc failed: {}, lowering failed {}; spills to stack: {}, spills to restore: {}, max spill slot {}",
      stats.lower_stats.reg_alloc_errors,
      stats.lower_stats.lowering_errors,
      stats.lower_stats.spills_to_slot,
      stats.lower_stats.spills_to_restore,
      stats.lower_stats.max_spill_slots_used
    );
  }

  if record_stats != RecordStats::None {
    // cpp: fopen(statsFile, "w") 失败时报固定消息 "Unable to open 'stats.json'"
    let write_result = File::create(&stats_file)
      .map(BufWriter::new)
      .and_then(|mut out| {
        let body = if record_stats == RecordStats::Total {
          serialize_compile_stats(&mut out, &stats)
        } else if stats_by_file {
          write_per_file_stats(&mut out, &files, &file_stats)
        } else {
          Ok(())
        };
        body.and_then(|()| out.flush())
      });

    if write_result.is_err() {
      eprintln!("Unable to open 'stats.json'");
      return 1;
    }
  }

  i32::from(failed != 0)
}

/// cpp `recordStats == File/Function` 分支: `{"<file>": <stats>, ...}`
fn write_per_file_stats<W: Write>(
  out: &mut W,
  files: &[String],
  file_stats: &[CompileStats],
) -> Result<()> {
  writeln!(out, "{{")?;

  let last = files.len().saturating_sub(1);
  for (i, (file, file_stat)) in files.iter().zip(file_stats.iter()).enumerate() {
    write!(out, "    \"{}\": ", escape_filename(file))?;
    serialize_compile_stats(out, file_stat)?;
    write!(out, "{}", if i == last { "\n" } else { ",\n" })?;
  }

  write!(out, "}}")
}
