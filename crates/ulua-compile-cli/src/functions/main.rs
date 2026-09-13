use alloc::{string::String, vec::Vec};
use core::ffi::{CStr, c_char, c_int};
use std::ffi::CString;

use ulua_cli_lib::functions::{
  get_source_files::get_source_files, set_luau_flags_default::set_luau_flags_default,
  set_luau_flags_flags_alt_b::set_luau_flags_c_char,
};
use ulua_code_gen::{
  enums::{function_stats_flags::FunctionStatsFlags, target::Target},
  records::lowering_stats::FUNCTION_STATS_ENABLE,
};
use ulua_common::{FFlag::DebugLuauTimeTracing, functions::assert_handler::assert_handler};

use crate::{
  enums::{compile_format::CompileFormat, record_stats::RecordStats},
  functions::{
    assertion_handler::assertion_handler,
    compile_file::compile_file,
    display_help::display_help,
    escape_filename::escape_filename,
    get_compile_format::get_compile_format,
    serialize_compile_stats::{CFile, serialize_compile_stats},
  },
  records::{compile_stats::CompileStats, global_options::GLOBAL_OPTIONS},
};

unsafe extern "C" {
  fn fopen(filename: *const c_char, mode: *const c_char) -> *mut CFile;
  fn fclose(stream: *mut CFile) -> c_int;
  fn fprintf(stream: *mut CFile, format: *const c_char, ...) -> c_int;
}

unsafe extern "C-unwind" fn assertion_handler_adapter(
  expr: *const c_char,
  file: *const c_char,
  line: i32,
  function: *const c_char,
) -> i32 {
  assertion_handler(expr, file, line, function)
}

fn atoi_like(value: &str) -> i32 {
  let bytes = value.as_bytes();
  let mut i = 0;

  while i < bytes.len() && bytes[i].is_ascii_whitespace() {
    i += 1;
  }

  let mut sign = 1i64;
  if i < bytes.len() {
    if bytes[i] == b'-' {
      sign = -1;
      i += 1;
    } else if bytes[i] == b'+' {
      i += 1;
    }
  }

  let mut result = 0i64;
  while i < bytes.len() && bytes[i].is_ascii_digit() {
    result = result
      .saturating_mul(10)
      .saturating_add((bytes[i] - b'0') as i64);
    i += 1;
  }

  (result.saturating_mul(sign)).clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

/// # Safety
///
/// `argv` 必须是指向长度至少为 `argc` 的有效 `*mut c_char` 数组指针，且各指针指向合法的以 NUL 结尾的 C 字符串。
pub unsafe fn main(argc: i32, argv: *mut *mut c_char) -> i32 {
  *assert_handler() = Some(assertion_handler_adapter);

  set_luau_flags_default();

  let mut compile_format = CompileFormat::Text;
  let mut assembly_target = Target::Host;
  let mut record_stats = RecordStats::None;
  let mut stats_file = String::from("stats.json");
  let mut bytecode_summary = false;
  let mut dump_constants = false;

  for i in 1..argc as usize {
    let arg_ptr = unsafe { *argv.add(i) };
    let arg = unsafe { CStr::from_ptr(arg_ptr).to_string_lossy().into_owned() };

    if arg == "-h" || arg == "--help" {
      display_help(unsafe { *argv });
      return 0;
    } else if let Some(suffix) = arg.strip_prefix("-O") {
      let level = atoi_like(suffix);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Optimization level must be between 0 and 2 inclusive.");
        return 1;
      }
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).optimization_level = level;
      }
    } else if let Some(suffix) = arg.strip_prefix("-g") {
      let level = atoi_like(suffix);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Debug level must be between 0 and 2 inclusive.");
        return 1;
      }
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).debug_level = level;
      }
    } else if let Some(suffix) = arg.strip_prefix("-t") {
      let level = atoi_like(suffix);
      if !(0..=1).contains(&level) {
        eprintln!("Error: Type info level must be between 0 and 1 inclusive.");
        return 1;
      }
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).type_info_level = level;
      }
    } else if let Some(value) = arg.strip_prefix("--target=") {
      if value == "a64" {
        assembly_target = Target::A64;
      } else if value == "a64_nf" {
        assembly_target = Target::A64NoFeatures;
      } else if value == "x64" {
        assembly_target = Target::X64SystemV;
      } else if value == "x64_ms" {
        assembly_target = Target::X64Windows;
      } else {
        eprintln!("Error: unknown target");
        return 1;
      }
    } else if arg == "--timetrace" {
      DebugLuauTimeTracing.set(true);
    } else if let Some(value) = arg.strip_prefix("--record-stats=") {
      if value == "total" {
        record_stats = RecordStats::Total;
      } else if value == "file" {
        record_stats = RecordStats::File;
      } else if value == "function" {
        record_stats = RecordStats::Function;
      } else {
        eprintln!("Error: unknown 'granularity' for '--record-stats'.");
        return 1;
      }
    } else if arg.starts_with("--bytecode-summary") {
      bytecode_summary = true;
    } else if arg == "--dump-constants" {
      dump_constants = true;
    } else if let Some(value) = arg.strip_prefix("--stats-file=") {
      stats_file = String::from(value);

      if stats_file.is_empty() {
        eprintln!("Error: filename missing for '--stats-file'.\n");
        return 1;
      }
    } else if arg.starts_with("--fflags=") {
      unsafe {
        set_luau_flags_c_char(arg_ptr.add(9));
      }
    } else if arg.starts_with("--vector-lib=") {
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).vector_lib = arg_ptr.add(13);
      }
    } else if arg.starts_with("--vector-ctor=") {
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).vector_ctor = arg_ptr.add(14);
      }
    } else if arg.starts_with("--vector-type=") {
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).vector_type = arg_ptr.add(14);
      }
    } else if arg.starts_with("--parse-cst") {
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).parse_cst = true;
      }
    } else if arg.starts_with("--only-parse") {
      unsafe {
        (*core::ptr::addr_of_mut!(GLOBAL_OPTIONS)).only_parse = true;
      }
    } else if let Some(format_name) = arg.strip_prefix("--") {
      if let Some(format) = get_compile_format(format_name) {
        compile_format = format;
      } else {
        eprintln!("Error: Unrecognized option '{}'.\n", arg);
        display_help(unsafe { *argv });
        return 1;
      }
    } else if arg.starts_with('-') {
      eprintln!("Error: Unrecognized option '{}'.\n", arg);
      display_help(unsafe { *argv });
      return 1;
    }
  }

  if bytecode_summary && record_stats != RecordStats::Function {
    eprintln!("'Error: Required '--record-stats=function' for '--bytecode-summary'.");
    return 1;
  }

  if DebugLuauTimeTracing.get() {
    eprintln!("To run with --timetrace, Luau has to be built with LUAU_ENABLE_TIME_TRACE enabled");
    return 1;
  }

  let files = unsafe { get_source_files(argc, argv) };
  let file_count = files.len();
  let mut stats = CompileStats::default();

  let mut file_stats: Vec<CompileStats> = Vec::new();
  if record_stats == RecordStats::File || record_stats == RecordStats::Function {
    file_stats.reserve(file_count);
  }

  let mut failed = 0;
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

    let ok = match CString::new(path.as_str()) {
      Ok(path_c) => compile_file(
        path_c.as_ptr(),
        compile_format,
        assembly_target,
        &mut file_stat,
        dump_constants,
      ),
      Err(_) => {
        eprintln!("Error opening {}", path);
        false
      }
    };

    if !ok {
      failed += 1;
    }

    stats += &file_stat;

    if record_stats == RecordStats::File || record_stats == RecordStats::Function {
      file_stats.push(file_stat);
    }
  }

  if compile_format == CompileFormat::Null {
    println!(
      "Compiled {} KLOC into {} KB bytecode (read {:.2}s, parse {:.2}s, compile {:.2}s)",
      (stats.lines / 1000) as i32,
      (stats.bytecode / 1024) as i32,
      stats.read_time,
      stats.parse_time,
      stats.compile_time
    );
  } else if compile_format == CompileFormat::CodegenNull {
    println!(
      "Compiled {} KLOC into {} KB bytecode => {} KB native code ({:.2}x) (read {:.2}s, parse {:.2}s, compile {:.2}s, codegen {:.2}s)",
      (stats.lines / 1000) as i32,
      (stats.bytecode / 1024) as i32,
      (stats.codegen / 1024) as i32,
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
    let stats_file_c = match CString::new(stats_file.as_str()) {
      Ok(stats_file_c) => stats_file_c,
      Err(_) => {
        eprintln!("Unable to open 'stats.json'");
        return 1;
      }
    };

    let fp = unsafe { fopen(stats_file_c.as_ptr(), c"w".as_ptr()) };

    if fp.is_null() {
      eprintln!("Unable to open 'stats.json'");
      return 1;
    }

    if record_stats == RecordStats::Total {
      unsafe {
        serialize_compile_stats(fp, &stats);
      }
    } else if record_stats == RecordStats::File || record_stats == RecordStats::Function {
      unsafe {
        fprintf(fp, c"{\n".as_ptr());
      }

      for i in 0..file_count {
        let escaped = escape_filename(&files[i]);
        let escaped_c = CString::new(escaped).expect("escaped filename contains NUL");
        unsafe {
          fprintf(fp, c"    \"%s\": ".as_ptr(), escaped_c.as_ptr());
          serialize_compile_stats(fp, &file_stats[i]);
          if i == file_count - 1 {
            fprintf(fp, c"\n".as_ptr());
          } else {
            fprintf(fp, c",\n".as_ptr());
          }
        }
      }

      unsafe {
        fprintf(fp, c"}".as_ptr());
      }
    }

    unsafe {
      fclose(fp);
    }
  }

  if failed != 0 { 1 } else { 0 }
}
