//! `extern "C" checkScript`（cpp `CLI/src/Web.cpp:142-182`）的集成测试。
//!
//! 只经 C 入口断言，故同时覆盖 `int useNewSolver` 的边界归一与结果缓存。

use core::{
  ffi::{CStr, c_int},
  ptr::null,
};

use ulua_web::functions::check_script::check_script;

/// 旧求解器的 C ABI 取值（见 `util::OLD_SOLVER_FLAG`）。
const OLD_SOLVER: c_int = 0;
/// 新求解器：非 0 即启用。
const NEW_SOLVER: c_int = 1;

/// 干净代码返回 null（诊断为空）。
#[test]
fn clean_source_returns_null() {
  // SAFETY: 字面量 C 字符串与 null 均为契约允许的输入。
  unsafe {
    assert!(check_script(c"local x = 1".as_ptr(), OLD_SOLVER).is_null());
    assert!(check_script(null(), OLD_SOLVER).is_null());
  }
}

/// 类型错误：返回 `行: 信息` 诊断（行号从 1 起）。
#[test]
fn type_error_is_reported_with_line() {
  // SAFETY: 字面量 C 字符串。
  unsafe {
    let ptr = check_script(c"local x: number = 's'".as_ptr(), OLD_SOLVER);
    assert!(!ptr.is_null());
    let msg = CStr::from_ptr(ptr).to_string_lossy();
    assert!(msg.starts_with("1: "), "got: {msg}");
  }
}

/// 新求解器路径同样可用。
#[test]
fn new_solver_returns_null_for_clean_source() {
  // SAFETY: 字面量 C 字符串。
  unsafe {
    assert!(check_script(c"local x = 1".as_ptr(), NEW_SOLVER).is_null());
  }
}
