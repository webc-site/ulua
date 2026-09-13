//! `extern "C" const char* checkScript(const char* source, int useNewSolver)`
//! (`CLI/src/Web.cpp:142-182`).
//!
//! The wasm type-checking entry point: builds a demo `Frontend`, registers the
//! Luau builtins, type-checks the single module `"main"`, and returns the
//! newline-joined `line: message` diagnostics (or null when clean). The C++
//! caches the result in a function-`static std::string` so the returned pointer
//! outlives the call; the Rust analog is a thread-local `CString`.

use alloc::string::{String, ToString};
use core::{
  any::Any,
  cell::RefCell,
  ffi::{CStr, c_char, c_int},
};
use std::{ffi::CString, panic::catch_unwind};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::{
    freeze::freeze, register_builtin_globals::register_builtin_globals,
    to_string_error::to_string_type_error, unfreeze::unfreeze,
  },
  records::{frontend::Frontend, frontend_options::FrontendOptions},
  type_aliases::module_name_type::ModuleName,
};

use crate::{
  records::{demo_config_resolver::DemoConfigResolver, demo_file_resolver::DemoFileResolver},
  util::cache_result,
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
pub(crate) fn run_check(source: &str, use_new_solver: c_int) -> String {
  let mut final_check_result = String::new();

  let mut file_resolver = DemoFileResolver::new();
  let mut config_resolver = DemoConfigResolver::new();
  let options = FrontendOptions::default();

  let mut frontend = Frontend::frontend_file_resolver_config_resolver_frontend_options(
    &mut file_resolver.base,
    &mut config_resolver.base,
    &options,
  );
  unsafe {
    frontend.wire_self_pointers();
  }

  frontend.set_luau_solver_mode(if use_new_solver != 0 {
    SolverMode::New
  } else {
    SolverMode::Old
  });

  // Add Luau builtins:
  //   Luau::unfreeze(frontend.globals.globalTypes);
  //   Luau::registerBuiltinGlobals(frontend, frontend.globals);
  //   Luau::freeze(frontend.globals.globalTypes);
  // 拆借：globals 是 frontend 的字段，register_builtin_globals 需要同时可变
  // 借用两者，经裸指针绕过借用检查。
  let frontend_ptr = &mut frontend as *mut Frontend;
  let globals = unsafe { &mut (*frontend_ptr).globals };
  unfreeze(globals.global_types_mut());
  // SAFETY: frontend_ptr 由局部变量 frontend 派生，整个调用期间 frontend 存活，
  // 且此期间不经过其他引用访问 frontend。
  unsafe { register_builtin_globals(&mut *frontend_ptr, globals, false) };
  freeze(globals.global_types_mut());

  // restart
  //   frontend.clear();
  //   fileResolver.source.clear();
  frontend.clear();
  file_resolver.source.clear();

  // fileResolver.source["main"] = source;
  file_resolver
    .source
    .insert(MAIN_MODULE.to_string(), source.to_string());

  // Luau::CheckResult checkResult = frontend.check("main");
  let check_result =
    frontend.check_module_name_optional_frontend_options(&ModuleName::from(MAIN_MODULE), None);

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
  unsafe {
    let source_str = if source.is_null() {
      String::new()
    } else {
      CStr::from_ptr(source).to_string_lossy().into_owned()
    };

    // try { ... } catch (const std::exception& e) { finalCheckResult = e.what(); }
    let final_check_result = match catch_unwind(move || run_check(&source_str, use_new_solver)) {
      Ok(result) => result,
      Err(payload) => panic_message(&payload),
    };

    FINAL_CHECK_RESULT.with(|r| cache_result(r, final_check_result))
  }
}

/// Extract a `std::exception::what()`-equivalent message from a caught panic
/// payload.
fn panic_message(payload: &(dyn Any + Send)) -> String {
  if let Some(s) = payload.downcast_ref::<&str>() {
    s.to_string()
  } else if let Some(s) = payload.downcast_ref::<String>() {
    s.clone()
  } else {
    "unknown error".to_string()
  }
}

#[cfg(test)]
mod tests {
  use core::{ffi::CStr, ptr::null};

  use super::{check_script, run_check};

  /// 干净代码：类型检查返回空串。
  #[test]
  fn clean_source_yields_no_diagnostics() {
    assert_eq!(run_check("local x = 1", 0), "");
  }

  /// 类型错误：返回 `行: 信息` 诊断（行号从 1 起）。
  #[test]
  fn type_error_is_reported_with_line() {
    let result = run_check("local x: number = 's'", 0);
    assert!(!result.is_empty(), "got: {result}");
    assert!(result.starts_with("1: "), "got: {result}");
  }

  /// 新求解器路径同样可用。
  #[test]
  fn new_solver_flag_is_accepted() {
    assert_eq!(run_check("local x = 1", 1), "");
  }

  /// C 入口：干净代码返回 null，诊断返回非 null。
  #[test]
  fn check_script_extern_contract() {
    // SAFETY: 字面量 C 字符串。
    unsafe {
      assert!(check_script(c"local x = 1".as_ptr(), 0).is_null());

      let ptr = check_script(c"local x: number = 's'".as_ptr(), 0);
      assert!(!ptr.is_null());
      let msg = CStr::from_ptr(ptr).to_string_lossy();
      assert!(msg.starts_with("1: "), "got: {msg}");

      // null 输入按空串处理，返回 null。
      assert!(check_script(null(), 0).is_null());
    }
  }
}
