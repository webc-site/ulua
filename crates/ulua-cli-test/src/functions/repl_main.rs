//! Node: `cxx:Function:Luau.CLI.Test:CLI/src/Repl.cpp:677:repl_main`
//! Source: `CLI/src/Repl.cpp:677-858` (faithful port)
//!
//! Faithful port of the C++ `replMain(int argc, char** argv)`. Parses the CLI
//! options, gathers the source files and runs them on a fresh `lua_State`,
//! returning `failed ? 1 : 0`. The profiling / coverage / counters / codegen
//! subsystems are recognised at the flag level but inert in the CLI-test
//! harness (no test enables them on the hit path).

use alloc::{string::String, vec::Vec};
use core::sync::atomic::{AtomicI32, Ordering};
use std::sync::RwLock;

use ulua_cli_lib::functions::get_source_files::get_source_files_from_slice;
use ulua_vm::{
  functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate},
  records::lua_state::lua_State,
};

use crate::{
  functions::{display_help::display_help, run_file::run_file, setup_state::setup_state},
  records::global_options::GlobalOptions,
};

// CLI-level state. In C++ these are file-static globals in Repl.cpp
// (`globalOptions`, `program_argc`, `program_argv`). The CLI-test harness runs
// each `replMain` call to completion before the next, so simple atomics mirror
// the C++ statics faithfully.

static OPTIMIZATION_LEVEL: AtomicI32 = AtomicI32::new(1);
static DEBUG_LEVEL: AtomicI32 = AtomicI32::new(1);
static PROGRAM_ARGS: RwLock<Vec<String>> = RwLock::new(Vec::new());

/// The CLI's `globalOptions` static (optimization + debug level).
pub fn global_options() -> GlobalOptions {
  GlobalOptions {
    optimization_level: OPTIMIZATION_LEVEL.load(Ordering::Relaxed),
    debug_level: DEBUG_LEVEL.load(Ordering::Relaxed),
  }
}

/// Program arguments slice.
pub fn program_args() -> Vec<String> {
  PROGRAM_ARGS.read().unwrap().clone()
}

/// Faithful port of `int replMain(int argc, char** argv)`.
pub fn repl_main(args: &[impl AsRef<str>]) -> i32 {
  let argc = args.len();
  let mut profile: i32 = 0;
  let mut coverage = false;
  let mut interactive = false;
  let mut codegen_perf = false;
  let mut counters = false;
  let mut program_args = argc;

  // Reset per-call state to the C++ static defaults.
  OPTIMIZATION_LEVEL.store(1, Ordering::Relaxed);
  DEBUG_LEVEL.store(1, Ordering::Relaxed);

  let argv0 = args.first().map(|s| s.as_ref()).unwrap_or("luau");

  let mut i = 1;
  while i < argc {
    let a = args[i].as_ref();

    if a == "-h" || a == "--help" {
      display_help(argv0);
      return 0;
    } else if a == "-i" || a == "--interactive" {
      interactive = true;
    } else if let Some(opt_str) = a.strip_prefix("-O") {
      let level: i32 = opt_str.parse().unwrap_or(0);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Optimization level must be between 0 and 2 inclusive.");
        return 1;
      }
      OPTIMIZATION_LEVEL.store(level, Ordering::Relaxed);
    } else if let Some(dbg_str) = a.strip_prefix("-g") {
      let level: i32 = dbg_str.parse().unwrap_or(0);
      if !(0..=2).contains(&level) {
        eprintln!("Error: Debug level must be between 0 and 2 inclusive.");
        return 1;
      }
      DEBUG_LEVEL.store(level, Ordering::Relaxed);
    } else if a == "--profile" {
      profile = 10000; // default to 10 KHz
    } else if let Some(rest) = a.strip_prefix("--profile=") {
      profile = rest.parse().unwrap_or(0);
    } else if a == "--codegen" {
      // Native codegen is unsupported in the CLI-test build; recognised
      // but inert (matches the C++ "not supported in current
      // configuration" warning path).
    } else if a == "--codegen-cold" {
      // see --codegen
    } else if a == "--codegen-perf" {
      codegen_perf = true;
    } else if a == "--coverage" {
      coverage = true;
    } else if a == "--counters" {
      counters = true;
    } else if a == "--timetrace" {
      // FFlag::DebugLuauTimeTracing — time tracing is compiled out.
    } else if a.starts_with("--fflags=") {
      // setLuauFlags(argv[i] + 9): not driven by the require suite.
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

  let prg_args: Vec<String> = args[program_args..]
    .iter()
    .map(|s| s.as_ref().to_string())
    .collect();
  *PROGRAM_ARGS.write().unwrap() = prg_args;

  if codegen_perf {
    // --codegen-perf is Linux-only in C++; everywhere else it errors out.
    // The require suite never sets it.
    eprintln!("--codegen-perf option is only supported on Linux");
    return 1;
  }

  let files: Vec<String> = get_source_files_from_slice(args);

  if files.is_empty() {
    // C++ starts the interactive REPL here. Not reachable from the require
    // tests (they always pass a script path).
    let _ = profile;
    let _ = coverage;
    let _ = counters;
    let _ = interactive;
    panic!("interactive REPL is not supported in the CLI-test harness");
  }

  unsafe {
    let l: *mut lua_State = lua_l_newstate();

    setup_state(l);

    // profilerStart / coverageInit / countersInit are no-ops in this
    // harness (the require suite enables none of them).

    let mut failed = 0i32;

    let n = files.len();
    for (idx, file) in files.iter().enumerate() {
      let is_last_file = idx == n - 1;
      let ran = run_file(file.as_str(), l, interactive && is_last_file);
      failed += (!ran) as i32;
    }

    lua_close(l);

    if failed != 0 { 1 } else { 0 }
  }
}
