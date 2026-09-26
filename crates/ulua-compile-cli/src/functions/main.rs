//! Source: `CLI/src/Compile.cpp:487-740` (`main`)
use alloc::{string::String, vec::Vec};
use std::{
  fs::File,
  io,
  io::{BufWriter, Result, Write},
};

use rayon::prelude::*;
use ulua_cli_lib::functions::{
  cli_preamble::cli_preamble, escape_filename::escape_filename,
  get_source_files::get_source_files_from_slice, parse_level_arg::apply_level_arg,
  report_unrecognized_option::report_unrecognized_option, set_luau_flags_flags::set_luau_flags,
  time_trace_unsupported::time_trace_unsupported, write_json_entries::write_json_entries,
};
use ulua_code_gen::{
  enums::{function_stats_flags::FunctionStatsFlags, target::Target},
  records::lowering_stats::FUNCTION_STATS_ENABLE,
};
use ulua_common::fflag::DebugLuauTimeTracing;

use crate::{
  enums::{compile_format::CompileFormat, record_stats::RecordStats},
  functions::{
    compile_file::{FileOutcome, compile_file},
    display_help::display_help,
    get_compile_format::get_compile_format,
    serialize_compile_stats::serialize_compile_stats,
  },
  records::{
    compile_stats::CompileStats,
    global_options::{mutate, snapshot},
  },
};

/// cpp 输出 `stats.lines / 1000` (KLOC)
const LINES_PER_KLOC: usize = 1000;

/// argv 值 → 堆上「NUL 结尾字节串」（`Vec<u8>` 末尾补 0），供 `copts()` 取
/// `as_ptr().cast()` 透传给编译器的 `*const c_char` 字段。argv 天然不含 NUL，
/// 无需 CString 的合法性校验与失败分支。
fn nul_terminated(value: &str) -> Vec<u8> {
  let mut bytes = Vec::with_capacity(value.len() + 1);
  bytes.extend_from_slice(value.as_bytes());
  bytes.push(0);
  bytes
}
/// cpp 输出 `stats.bytecode / 1024` (KB)
const BYTES_PER_KB: usize = 1024;
/// `--stats-file` 缺省文件名（help 文案与此处默认值共用，防漂移）
pub(crate) const DEFAULT_STATS_FILE: &str = "stats.json";

/// cpp `int main(int argc, char** argv)` (CLI/src/Compile.cpp:487-740)
pub fn run(args: &[String]) -> i32 {
  // argv0 → 断言处理器 → 默认旗标收敛于共享前奏 `cli_preamble`。compile 的 `--help`
  // 在下方逐参循环里识别（额外支持 `-h` 且扫描全部参数），故这里只取 `argv0`、
  // 丢弃 `help_requested` 标记，早退语义完全交给原有循环。
  let argv0 = cli_preamble(args, "ulua-compile").argv0;

  let mut compile_format = CompileFormat::Text;
  let mut assembly_target = Target::Host;
  let mut record_stats = RecordStats::None;
  let mut stats_file = String::from(DEFAULT_STATS_FILE);
  let mut bytecode_summary = false;
  let mut dump_constants = false;
  let mut dump_reg_spills = false;

  for arg in args.iter().skip(1).map(String::as_str) {
    if arg == "-h" || arg == "--help" {
      display_help(argv0);
      return 0;
    } else if let Some(suffix) = arg.strip_prefix("-O") {
      if !apply_level_arg(suffix, 0, 2, "Optimization", |level| {
        mutate(|opts| opts.optimization_level = level)
      }) {
        return 1;
      }
    } else if let Some(suffix) = arg.strip_prefix("-g") {
      if !apply_level_arg(suffix, 0, 2, "Debug", |level| {
        mutate(|opts| opts.debug_level = level)
      }) {
        return 1;
      }
    } else if let Some(suffix) = arg.strip_prefix("-t") {
      if !apply_level_arg(suffix, 0, 1, "Type info", |level| {
        mutate(|opts| opts.type_info_level = level)
      }) {
        return 1;
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
      // cpp 记入 globalOptions.dumpRegSpills 并透传给 options.includeRegSpills;
      // ulua-code-gen 尚无该字段 (见下方 TODO(cross-crate)), 故此处只记局部量, 解析后统一提示
      dump_reg_spills = true;
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
      mutate(|opts| opts.vector_lib = Some(nul_terminated(value)));
    } else if let Some(value) = arg.strip_prefix("--vector-ctor=") {
      mutate(|opts| opts.vector_ctor = Some(nul_terminated(value)));
    } else if let Some(value) = arg.strip_prefix("--vector-type=") {
      mutate(|opts| opts.vector_type = Some(nul_terminated(value)));
    } else if arg.starts_with("--parse-cst") {
      mutate(|opts| opts.parse_cst = true);
    } else if arg.starts_with("--only-parse") {
      mutate(|opts| opts.only_parse = true);
    } else if arg.starts_with('-') {
      // 未消费的选项：`--<mode>` 命中即设输出格式，否则（未知长选项与裸 `-x`）同路报错
      if let Some(format) = arg.strip_prefix("--").and_then(get_compile_format) {
        compile_format = format;
      } else {
        report_unrecognized_option(arg);
        display_help(argv0);
        return 1;
      }
    }
  }

  if bytecode_summary && record_stats != RecordStats::Function {
    eprintln!("'Error: Required '--record-stats=function' for '--bytecode-summary'.");
    return 1;
  }

  // ISSUE[code-gen.include_reg_spills] TODO(cross-crate): cpp 的
  // `options.includeRegSpills = globalOptions.dumpRegSpills` 唯一消费点是
  // ulua-code-gen 的 `AssemblyOptions::include_reg_spills`（含 regalloc 日志子系统），
  // 该字段/子系统尚未移植，故无法透传（登记于 docs/CONFORMANCE.md「Known gaps」，
  // 锚点同名）。在此之前显式告知用户旗标未生效，处理方式与 analyze-cli 的 `-j`
  // 一致，避免静默接受。
  if dump_reg_spills {
    eprintln!(
      "note: --dump-regspills is not effective in this build; codegen does not log register spills yet"
    );
  }

  // cpp: LUAU_ENABLE_TIME_TRACE 未定义时的守卫
  if time_trace_unsupported() {
    return 1;
  }

  let files = get_source_files_from_slice(args);

  // 参数解析完成后的全局选项快照：rayon 工作线程的 thread_local 是默认值，
  // 每个编译任务入口先在自身线程上重放本快照（见 compile_file / global_options::restore）。
  let globals = snapshot();

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

  // 编译与输出分两步：先按文件产出 FileOutcome（可并行），再按 files
  // 原顺序统一落 stdout/stderr 并聚合 stats——输出字节序与串行版一致。
  //
  // CompileFormat::Codegen*（JIT 汇编/IR/机器码）走 ulua-code-gen 的深层
  // unsafe lowering 链（get_assembly 及其 callee 携带裸指针契约），正确优先：
  // 该分支保持逐文件串行；其余格式零跨文件共享状态，rayon 并行。
  // par_iter/iter 的 map 均保输入序，collect::<Vec<_>>() 结果与 files 对齐。
  let codegen_serial = matches!(
    compile_format,
    CompileFormat::Codegen
      | CompileFormat::CodegenAsm
      | CompileFormat::CodegenIr
      | CompileFormat::CodegenVerbose
      | CompileFormat::CodegenNull
  );
  let outcomes: Vec<FileOutcome> = if codegen_serial {
    files
      .iter()
      .map(|path| {
        compile_file(
          path,
          compile_format,
          assembly_target,
          &globals,
          function_stats,
          dump_constants,
        )
      })
      .collect()
  } else {
    files
      .par_iter()
      .map(|path| {
        compile_file(
          path,
          compile_format,
          assembly_target,
          &globals,
          function_stats,
          dump_constants,
        )
      })
      .collect()
  };

  let mut stats = CompileStats::default();
  for outcome in outcomes {
    let FileOutcome {
      ok,
      stdout,
      stderr,
      stats: file_stat,
    } = outcome;

    if !stdout.is_empty() {
      let _ = io::stdout().write_all(&stdout);
    }
    if !stderr.is_empty() {
      eprint!("{stderr}");
    }

    if !ok {
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
    // cpp: `fopen(statsFile, "w")` 失败才报错，且消息硬编码为 'stats.json'（忽略
    // --stats-file 的自定义名），序列化/写盘的 fprintf 返回值被丢弃。Rust 侧按真实原因
    // 分支报错并使用真实文件名：把「磁盘满/写失败」误报成「打不开」会误导用户。
    let mut out = match File::create(&stats_file) {
      Ok(file) => BufWriter::new(file),
      Err(error) => {
        eprintln!("Unable to open '{stats_file}': {error}");
        return 1;
      }
    };

    let write_result = if record_stats == RecordStats::Total {
      serialize_compile_stats(&mut out, &stats).and_then(|()| out.flush())
    } else if stats_by_file {
      write_per_file_stats(&mut out, &files, &file_stats).and_then(|()| out.flush())
    } else {
      Ok(())
    };

    if let Err(error) = write_result {
      eprintln!("Unable to write '{stats_file}': {error}");
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

  write_json_entries(
    out,
    files.iter().zip(file_stats.iter()),
    |out, (file, file_stat)| {
      write!(out, "    \"{}\": ", escape_filename(file))?;
      serialize_compile_stats(out, file_stat)
    },
  )?;

  write!(out, "}}")
}
