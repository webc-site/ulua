//! `extern "C" const char* checkScript(const char* source, int useNewSolver)`
//! (`CLI/src/Web.cpp:142-182`).
//!
//! The wasm type-checking entry point: builds a demo `Frontend`, registers the
//! Luau builtins, type-checks the single module `"main"`, and returns the
//! newline-joined `line: message` diagnostics (or null when clean). The C++
//! caches the result in a function-`static std::string` so the returned pointer
//! outlives the call; the Rust analog is a thread-local `CString`.

use core::{
  any::Any,
  cell::RefCell,
  ffi::{c_char, c_int},
};
use std::{
  ffi::CString,
  panic::catch_unwind,
  string::{String, ToString},
};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{
    freeze::freeze, register_builtin_globals::register_builtin_globals,
    to_string_error::to_string_type_error, unfreeze::unfreeze,
  },
  records::{file_resolver::FileResolver, frontend::Frontend, frontend_options::FrontendOptions},
  type_aliases::module_name_type::ModuleName,
};

use crate::{
  records::{demo_config_resolver::DemoConfigResolver, demo_file_resolver::DemoFileResolver},
  util::{OLD_SOLVER_FLAG, cache_result, cstr_cow},
};

/// Web.cpp 里写死的单模块名。
const MAIN_MODULE: &str = "main";

thread_local! {
    /// Mirror of the C++ `static std::string finalCheckResult;` — keeps the
    /// returned C string alive after the call returns.
    static FINAL_CHECK_RESULT: RefCell<Option<CString>> = const { RefCell::new(None) };
}

/// The fallible body, run under `catch_unwind` to reproduce the C++
/// `try { ... } catch (const std::exception& e) { finalCheckResult = e.what(); }`.
///
/// 纯 Rust 内部入口：求解器选择用 `bool` 表达（C ABI 的 `c_int` 语义在
/// `check_script` 入口处归一）。
pub(crate) fn run_check(source: &str, use_new_solver: bool) -> String {
  let mut final_check_result = String::new();

  let mut file_resolver = DemoFileResolver::new();
  let mut config_resolver = DemoConfigResolver::new();
  let options = FrontendOptions::default();

  let mut frontend = Frontend::frontend_file_resolver_config_resolver_frontend_options(
    &mut file_resolver as *mut dyn FileResolver,
    &mut config_resolver.base,
    &options,
  );
  // SAFETY: frontend 已置于最终栈位置，wire 后不得再移动；此调用把内部指针
  // 指回本 frontend 的子对象。
  unsafe {
    frontend.wire_self_pointers();
  }

  frontend.set_luau_solver_mode(if use_new_solver {
    SolverMode::New
  } else {
    SolverMode::Old
  });

  // Add Luau builtins:
  //   Luau::unfreeze(frontend.globals.globalTypes);
  //   Luau::registerBuiltinGlobals(frontend, frontend.globals);
  //   Luau::freeze(frontend.globals.globalTypes);
  // 拆借：register_builtin_globals 需要同时可变借用 frontend 与其 globals 字段，
  // 经裸指针内联派生（不物化长期 &mut），与 ulua-rt typecheck 的写法一致。
  let frontend_ptr = &mut frontend as *mut Frontend;
  unsafe {
    unfreeze((*frontend_ptr).globals.global_types_mut());
    // SAFETY: frontend_ptr 由局部变量 frontend 派生，整个调用期间 frontend 存活，
    // 且此期间不经过其他引用访问 frontend。
    register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
    freeze((*frontend_ptr).globals.global_types_mut());
  }

  // restart
  //   frontend.clear();
  //   fileResolver.source.clear();
  frontend.clear();
  file_resolver.source.clear();

  // fileResolver.source["main"] = source;
  let main_module: ModuleName = MAIN_MODULE.into();
  file_resolver
    .source
    .insert(main_module.clone(), source.to_string());

  // Luau::CheckResult checkResult = frontend.check("main");
  let check_result = frontend.check_module_name_optional_frontend_options(&main_module, None);

  for err in &check_result.errors {
    if !final_check_result.is_empty() {
      final_check_result.push('\n');
    }
    // std::to_string(err.location.begin.line + 1)
    final_check_result.push_str(&(err.location.begin.line + 1).to_string());
    final_check_result.push_str(": ");
    // Luau::to_string(err)
    final_check_result.push_str(&to_string_type_error(err));
  }

  final_check_result
}

/// # Safety
/// `source` must be a valid, NUL-terminated C string (the wasm/JS caller's
/// contract), or null.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C-unwind" fn check_script(
  source: *const c_char,
  use_new_solver: c_int,
) -> *const c_char {
  // SAFETY: source 满足 C 入口契约（NUL 结尾或 null），调用期间由调用方持有。
  let source_str = unsafe { cstr_cow(source) };

  // try { ... } catch (const std::exception& e) { finalCheckResult = e.what(); }
  // C ABI 的 `int useNewSolver` 在边界处归一为 bool（非 0 即新求解器）。
  let final_check_result =
    match catch_unwind(|| run_check(&source_str, use_new_solver != OLD_SOLVER_FLAG)) {
      Ok(result) => result,
      Err(payload) => panic_message(&payload),
    };

  FINAL_CHECK_RESULT.with(|r| cache_result(r, final_check_result))
}

/// panic 载荷既非 `&str` 也非 `String` 时的兜底文案。
const UNKNOWN_ERROR: &str = "unknown error";

/// Extract a `std::exception::what()`-equivalent message from a caught panic
/// payload.
fn panic_message(payload: &(dyn Any + Send)) -> String {
  if let Some(s) = payload.downcast_ref::<&str>() {
    s.to_string()
  } else if let Some(s) = payload.downcast_ref::<String>() {
    s.clone()
  } else {
    UNKNOWN_ERROR.to_string()
  }
}
