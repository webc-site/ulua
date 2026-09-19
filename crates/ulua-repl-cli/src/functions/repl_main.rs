//! Faithful port of the C++ `int replMain(int argc, char** argv)` from
//! `CLI/src/Repl.cpp`. Parses the CLI options, installs the assertion handler,
//! then either starts the interactive REPL (no file arguments) or runs each
//! source file on a fresh `lua_State`, optionally enabling profiling / coverage
//! / counters / native codegen, returning `failed ? 1 : 0`.

use alloc::{string::String, vec::Vec};
use core::sync::atomic::{AtomicBool, Ordering};

use ulua_cli_lib::{
  functions::{
    assertion_handler::install_assertion_handler, atoi::atoi, display_help::display_help,
    get_source_files::get_source_files_from_slice, parse_level_arg::parse_level_arg,
    set_luau_flags_flags_alt_b::set_luau_flags, time_trace_unsupported::time_trace_unsupported,
  },
  records::{
    global_options::{reset_to_defaults, set_debug_level, set_optimization_level},
    lua_state_guard::LuaStateGuard,
  },
};
use ulua_code_gen::functions::is_supported::is_supported;
use ulua_common::fflag::DebugLuauTimeTracing;
use ulua_vm::{functions::lua_l_newstate::lua_l_newstate, type_aliases::lua_state::lua_State};

use crate::functions::{
  counters_dump::counters_dump, counters_init::counters_init, coverage_dump::coverage_dump,
  coverage_init::coverage_init, profiler_dump::profiler_dump, profiler_start::profiler_start,
  profiler_stop::profiler_stop, run_file::run_file, run_repl::run_repl, setup_state::setup_state,
};

// CLI-level static from Repl.cpp: `static bool codegen`. `program_argc/argv`
// 不保留为静态：参数区间在 replMain 里算出后直接传给 runFile。
// cpp 的 `codegenCold` / `jitInliner` 静态不保留：cold codegen 与 JitInliner
// 尚未移植（跨 crate 缺口），对应 flag 仅保持解析一致（见下方分支）。
static REPL_CODEGEN: AtomicBool = AtomicBool::new(false);

/// `static bool codegen` accessor — used by setupState, the requirer and runFile.
pub(crate) fn repl_codegen_enabled() -> bool {
  REPL_CODEGEN.load(Ordering::Relaxed)
}

pub fn repl_main(args: &[impl AsRef<str>]) -> i32 {
  // Luau::assertHandler() = assertionHandler;
  install_assertion_handler();

  // (Windows) SetConsoleOutputCP(CP_UTF8) — not applicable on this build.

  let argc = args.len();
  let mut profile: i32 = 0;
  let mut coverage = false;
  let mut interactive = false;
  let mut codegen_perf = false;
  let mut counters = false;
  let mut program_argc = argc;

  // Reset the CLI statics to the C++ defaults for this invocation.
  REPL_CODEGEN.store(false, Ordering::Relaxed);
  reset_to_defaults();

  let argv0 = args.first().map(|s| s.as_ref()).unwrap_or("luau");

  for (idx, arg) in args.iter().enumerate().skip(1) {
    let a = arg.as_ref();

    if a == "-h" || a == "--help" {
      display_help(argv0);
      return 0;
    } else if a == "-i" || a == "--interactive" {
      interactive = true;
    } else if let Some(suffix) = a.strip_prefix("-O") {
      // atoi(argv[i] + 2): parse leading digits, defaulting to 0.
      match parse_level_arg(suffix, 0, 2, "Optimization") {
        Some(level) => set_optimization_level(level),
        None => return 1,
      }
    } else if let Some(suffix) = a.strip_prefix("-g") {
      match parse_level_arg(suffix, 0, 2, "Debug") {
        Some(level) => set_debug_level(level),
        None => return 1,
      }
    } else if a == "--profile" {
      // default to 10 KHz
      profile = 10000;
    } else if let Some(rest) = a.strip_prefix("--profile=") {
      profile = atoi(rest);
    } else if a == "--codegen" {
      REPL_CODEGEN.store(true, Ordering::Relaxed);
    } else if a == "--codegen-cold" {
      // cpp: codegen = true; codegenCold = true; — cold 路径未移植，
      // 仅保持 codegen 生效与解析一致
      REPL_CODEGEN.store(true, Ordering::Relaxed);
    } else if a == "--codegen-perf" {
      REPL_CODEGEN.store(true, Ordering::Relaxed);
      codegen_perf = true;
    } else if a == "--coverage" {
      coverage = true;
    } else if a == "--counters" {
      counters = true;
    } else if a == "--timetrace" {
      // SAFETY: 启动期解析 --timetracing 时写入，先于任何 VM 线程
      unsafe { DebugLuauTimeTracing.set(true) };
    } else if let Some(flags) = a.strip_prefix("--fflags=") {
      set_luau_flags(flags);
    } else if a == "--jit-inliner" {
      // cpp: jitInliner = true; — Luau::JitInliner 未移植（跨 crate 缺口），
      // 此处仅保持解析与 cpp 一致
    } else if a == "--program-args" || a == "-a" {
      program_argc = idx + 1;
      break;
    } else if a.starts_with('-') {
      // cpp 消息以 "\n\n" 结尾
      eprintln!("Error: Unrecognized option '{}'.\n", a);
      display_help(argv0);
      return 1;
    }
  }

  // cpp 的 `program_argv = argv + program_argc`：切片即可，零拷贝传给 runFile。
  let program_args = &args[program_argc..];

  // #if !defined(LUAU_ENABLE_TIME_TRACE): time tracing is compiled out.
  if time_trace_unsupported() {
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
    // cpp Repl.cpp:844 `unique_ptr<lua_State, void (*)(lua_State*)>` —— 由守卫关闭
    let global_state = LuaStateGuard(lua_l_newstate());
    let l: *mut lua_State = global_state.0;

    unsafe {
      setup_state(l);
    }

    if profile != 0 {
      profiler_start(l, profile);
    }

    if coverage {
      coverage_init(l);
    }

    if counters {
      counters_init(l);
    }

    let mut failed = 0i32;

    let n = files.len();
    for (idx, file) in files.iter().enumerate() {
      let is_last_file = idx == n - 1;
      let ran = unsafe { run_file(file, l, interactive && is_last_file, program_args) };
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

    // 守卫在块尾 drop → lua_close，早于任何 return/panic 逃逸
    if failed != 0 { 1 } else { 0 }
  }
}
