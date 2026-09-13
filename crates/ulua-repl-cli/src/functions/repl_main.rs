//! Faithful port of the C++ `int replMain(int argc, char** argv)` from
//! `CLI/src/Repl.cpp`. Parses the CLI options, installs the assertion handler,
//! then either starts the interactive REPL (no file arguments) or runs each
//! source file on a fresh `lua_State`, optionally enabling profiling / coverage
//! / counters / native codegen, returning `failed ? 1 : 0`.

use alloc::{string::String, vec::Vec};
use core::{
  ffi::{c_char, c_void},
  sync::atomic::{AtomicBool, Ordering},
};
use std::sync::RwLock;

use ulua_cli_lib::functions::{
  get_source_files::get_source_files_from_slice, set_luau_flags_flags_alt_b::set_luau_flags,
};
use ulua_code_gen::functions::is_supported::is_supported;
use ulua_common::{FFlag::DebugLuauTimeTracing, functions::assert_handler::assert_handler};
use ulua_vm::{
  functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate},
  records::lua_state::lua_State,
};

use crate::{
  functions::{
    assertion_handler::assertion_handler, counters_dump::counters_dump,
    counters_init::counters_init, coverage_dump::coverage_dump, coverage_init::coverage_init,
    display_help::display_help, profiler_dump::profiler_dump, profiler_start::profiler_start,
    profiler_stop::profiler_stop, run_file::run_file, run_repl::run_repl, setup_state::setup_state,
  },
  records::global_options::GlobalOptions,
};

// CLI-level statics from Repl.cpp: `static bool codegen`, `static bool
// codegenCold`, `static int program_argc`, `char** program_argv`.
static REPL_CODEGEN: AtomicBool = AtomicBool::new(false);
static REPL_CODEGEN_COLD: AtomicBool = AtomicBool::new(false);
static PROGRAM_ARGS: RwLock<Vec<String>> = RwLock::new(Vec::new());

/// `static bool codegen` accessor — used by setupState, the requirer and runFile.
pub fn repl_codegen_enabled() -> bool {
  REPL_CODEGEN.load(Ordering::Relaxed)
}

/// Program arguments accessor — used by runFile.
pub fn program_args() -> Vec<String> {
  PROGRAM_ARGS.read().unwrap().clone()
}

pub(crate) static mut GLOBAL_OPTIONS: GlobalOptions = GlobalOptions {
  optimization_level: 1,
  debug_level: 1,
};

pub fn repl_main(args: &[impl AsRef<str>]) -> i32 {
  // Luau::assertHandler() = assertionHandler;
  *assert_handler() = Some(assertion_handler_adapter);

  // (Windows) SetConsoleOutputCP(CP_UTF8) — not applicable on this build.

  let argc = args.len();
  let mut profile: i32 = 0;
  let mut coverage = false;
  let mut interactive = false;
  let mut codegen_perf = false;
  let mut counters = false;
  let mut program_args = argc;

  // Reset the CLI statics to the C++ defaults for this invocation.
  REPL_CODEGEN.store(false, Ordering::Relaxed);
  REPL_CODEGEN_COLD.store(false, Ordering::Relaxed);
  unsafe {
    GLOBAL_OPTIONS = GlobalOptions {
      optimization_level: 1,
      debug_level: 1,
    };
  }

  let argv0 = args.first().map(|s| s.as_ref()).unwrap_or("luau");

  let mut i = 1usize;
  while i < argc {
    let a = args[i].as_ref();

    if a == "-h" || a == "--help" {
      display_help(argv0);
      return 0;
    } else if a == "-i" || a == "--interactive" {
      interactive = true;
    } else if let Some(suffix) = a.strip_prefix("-O") {
      // atoi(argv[i] + 2): parse leading digits, defaulting to 0.
      let level = atoi_like(suffix);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Optimization level must be between 0 and 2 inclusive.");
        return 1;
      }
      unsafe {
        GLOBAL_OPTIONS.optimization_level = level;
      }
    } else if let Some(suffix) = a.strip_prefix("-g") {
      let level = atoi_like(suffix);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Debug level must be between 0 and 2 inclusive.");
        return 1;
      }
      unsafe {
        GLOBAL_OPTIONS.debug_level = level;
      }
    } else if a == "--profile" {
      profile = 10000; // default to 10 KHz
    } else if let Some(rest) = a.strip_prefix("--profile=") {
      profile = atoi_like(rest);
    } else if a == "--codegen" {
      REPL_CODEGEN.store(true, Ordering::Relaxed);
    } else if a == "--codegen-cold" {
      REPL_CODEGEN.store(true, Ordering::Relaxed);
      REPL_CODEGEN_COLD.store(true, Ordering::Relaxed);
    } else if a == "--codegen-perf" {
      REPL_CODEGEN.store(true, Ordering::Relaxed);
      codegen_perf = true;
    } else if a == "--coverage" {
      coverage = true;
    } else if a == "--counters" {
      counters = true;
    } else if a == "--timetrace" {
      DebugLuauTimeTracing.set(true);
    } else if let Some(flags) = a.strip_prefix("--fflags=") {
      set_luau_flags(flags);
    } else if a == "--program-args" || a == "-a" {
      program_args = i + 1;
      break;
    } else if a.starts_with('-') {
      eprintln!("Error: Unrecognized option '{}'.\n", a);
      display_help(argv0);
      return 1;
    }

    i += 1;
  }

  *PROGRAM_ARGS.write().unwrap() = args[program_args..]
    .iter()
    .map(|s| s.as_ref().to_string())
    .collect();

  // #if !defined(LUAU_ENABLE_TIME_TRACE): time tracing is compiled out.
  if DebugLuauTimeTracing.get() {
    eprintln!("To run with --timetrace, Luau has to be built with LUAU_ENABLE_TIME_TRACE enabled");
    return 1;
  }

  if codegen_perf {
    // The --codegen-perf perf-map path is Linux-only in C++; on other
    // platforms it errors out. The Rust codegen port does not expose
    // CodeGen::setPerfLog, so we take the unsupported-platform branch.
    eprintln!("--codegen-perf option is only supported on Linux");
    return 1;
  }

  if repl_codegen_enabled() && !is_supported() {
    eprintln!("Warning: Native code generation is not supported in current configuration");
  }

  let files: Vec<String> = get_source_files_from_slice(args);

  if files.is_empty() {
    unsafe {
      run_repl();
    }
    0
  } else {
    unsafe {
      let l: *mut lua_State = lua_l_newstate();

      setup_state(l);

      if profile != 0 {
        profiler_start(l, profile);
      }

      if coverage {
        coverage_init(l);
      }

      if counters {
        counters_init(l as *mut c_void);
      }

      let mut failed = 0i32;

      let n = files.len();
      for (idx, file) in files.iter().enumerate() {
        let is_last_file = idx == n - 1;
        let ran = run_file(file, l, interactive && is_last_file);
        failed += (!ran) as i32;
      }

      if profile != 0 {
        profiler_stop();
        profiler_dump("profile.out");
      }

      if coverage {
        coverage_dump("coverage.out");
      }

      if counters {
        counters_dump("callgrind.out");
      }

      lua_close(l);

      if failed != 0 { 1 } else { 0 }
    }
  }
}

// Adapter matching the AssertHandler fn-pointer ABI expected by Common.
unsafe extern "C-unwind" fn assertion_handler_adapter(
  expr: *const c_char,
  file: *const c_char,
  line: i32,
  function: *const c_char,
) -> i32 {
  unsafe { assertion_handler(expr, file, line, function) }
}

// Mirrors C's atoi(s): parse the leading optional sign + digits, ignoring any
// trailing characters; non-numeric input yields 0.
fn atoi_like(s: &str) -> i32 {
  let bytes = s.as_bytes();
  let mut idx = 0;
  let mut sign = 1i32;

  if idx < bytes.len() && (bytes[idx] == b'+' || bytes[idx] == b'-') {
    if bytes[idx] == b'-' {
      sign = -1;
    }
    idx += 1;
  }

  let mut value: i32 = 0;
  while idx < bytes.len() && bytes[idx].is_ascii_digit() {
    value = value
      .wrapping_mul(10)
      .wrapping_add((bytes[idx] - b'0') as i32);
    idx += 1;
  }

  sign * value
}
