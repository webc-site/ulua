//! Faithful port of the C++ `int replMain(int argc, char** argv)` from
//! `CLI/src/Repl.cpp`. Parses the CLI options, installs the assertion handler,
//! then either starts the interactive REPL (no file arguments) or runs each
//! source file on a fresh `LuaState`, optionally enabling profiling / coverage
//! / counters / native codegen, returning `failed ? 1 : 0`.

use alloc::{string::String, vec::Vec};
use core::sync::atomic::{AtomicBool, Ordering};

use ulua_cli_lib::{
  functions::{
    argv::argv0, assertion_handler::install_assertion_handler, atoi::atoi,
    display_help::display_help, get_source_files::get_source_files_from_slice,
    parse_level_arg::apply_level_arg, report_unrecognized_option::report_unrecognized_option,
    set_luau_flags_flags::set_luau_flags, time_trace_unsupported::time_trace_unsupported,
  },
  records::global_options::{reset_to_defaults, set_debug_level, set_optimization_level},
};
use ulua_code_gen::functions::is_supported::is_supported;
use ulua_common::fflag::DebugLuauTimeTracing;
use ulua_vm::{
  functions::lua_l_newstate::lua_l_newstate,
  records::{lua_state::LuaState, lua_state_guard::LuaStateGuard},
};

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

  let argv0 = argv0(args, "luau");

  for (idx, arg) in args.iter().enumerate().skip(1) {
    let a = arg.as_ref();

    if a == "-h" || a == "--help" {
      display_help(argv0);
      return 0;
    } else if a == "-i" || a == "--interactive" {
      interactive = true;
    } else if let Some(suffix) = a.strip_prefix("-O") {
      // atoi(argv[i] + 2): parse leading digits, defaulting to 0.
      if !apply_level_arg(suffix, 0, 2, "Optimization", set_optimization_level) {
        return 1;
      }
    } else if let Some(suffix) = a.strip_prefix("-g") {
      if !apply_level_arg(suffix, 0, 2, "Debug", set_debug_level) {
        return 1;
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
      DebugLuauTimeTracing.set(true);
    } else if let Some(flags) = a.strip_prefix("--fflags=") {
      set_luau_flags(flags);
    } else if a == "--jit-inliner" {
      // cpp: jitInliner = true; — Luau::JitInliner 未移植（跨 crate 缺口），
      // 此处仅保持解析与 cpp 一致
    } else if a == "--program-args" || a == "-a" {
      program_argc = idx + 1;
      break;
    } else if a.starts_with('-') {
      report_unrecognized_option(a);
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
    // Safety: run_repl 的 /// # Safety（进程级信号处理与单线程驱动）在 main 入口路径成立：此刻无其它 REPL/VM 驱动者。
    unsafe {
      run_repl();
    }
    0
  } else {
    // cpp Repl.cpp:844 `unique_ptr<LuaState, void (*)(LuaState*)>` —— 由守卫关闭
    let global_state = LuaStateGuard(lua_l_newstate());
    let l: *mut LuaState = global_state.0;

    // Safety: l 是刚由 lua_l_newstate 创建、被 LuaStateGuard 持有（作用域退出才 close）的新状态，setup_state 的契约（刚创建、有效）满足；仅当进程级 OOM 时该函数返回 null，此时行为与 cpp oracle（unique_ptr(lua_newstate()) 同样不判空）一致——继承上游边界而非本 port 新增隐患。
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
      // Safety: l 在守卫作用域内存活（close 在循环之后），run_file 的 gl 契约满足；file/program_args 借自本帧 Vec 迭代、调用窗口内不失效。
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
