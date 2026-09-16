//! `extern "C" const char* executeScript(const char* source)`
//! (`CLI/src/Web.cpp:184-208`).
//!
//! The wasm entry point: enables every `Luau*` bool fast flag, spins up a fresh
//! sandboxed Lua state, runs the script via [`run_code`], and returns the result
//! string (or null when empty). The C++ caches the result in a function-`static`
//! `std::string` so the returned pointer outlives the call; the Rust analog is a
//! thread-local `CString`.

use core::{cell::RefCell, ffi::c_char};
use std::ffi::CString;

use ulua_common::set_luau_bool_flags;
use ulua_vm::{
  functions::{
    lua_close::lua_close, lua_l_newstate::lua_l_newstate, lua_l_sandboxthread::lua_l_sandboxthread,
  },
  type_aliases::lua_state::lua_State,
};

use crate::{
  functions::{run_code::run_code, setup_state::setup_state},
  util::{cache_result, cstr_cow},
};

thread_local! {
    /// Mirror of the C++ `static std::string result;` — keeps the returned
    /// C string alive after the call returns.
    static RESULT: RefCell<Option<CString>> = const { RefCell::new(None) };
}

/// # Safety
/// `source` must be a valid, NUL-terminated C string (the wasm/JS caller's
/// contract), or null.
#[cfg_attr(not(test), unsafe(no_mangle))]
pub unsafe extern "C-unwind" fn execute_script(source: *const c_char) -> *const c_char {
  // SAFETY: source 满足 C 入口契约（NUL 结尾或 null），调用期间由调用方持有；
  // cstr_cow 自身是安全函数，内部 unsafe 已收敛。
  let source_str = unsafe { cstr_cow(source) };

  // setup flags: 同 cpp `setLuauFlagsDefault()` —— 仅开启 `Luau*` 前缀且
  // 非实验性的 bool FastFlag（见 `is_default_enabled_flag`）。
  set_luau_bool_flags(true);

  // create new state + setup state + sandbox thread + run code,
  // 直至 unique_ptr 析构 (lua_close) —— unsafe 范围收敛到 VM 调用序列
  let result = unsafe {
    // unique_ptr<lua_State, lua_close> globalState(luaL_newstate(), lua_close);
    let l: *mut lua_State = lua_l_newstate();

    // setup state
    setup_state(l);

    // sandbox thread
    lua_l_sandboxthread(l);

    // run code + collect error
    // cstr_cow：合法 UTF-8 时零拷贝；原 `str::from_utf8_unchecked` 在非
    // UTF-8 输入下是 UB，已修复。
    let result = run_code(l, &source_str);

    // unique_ptr destructor: lua_close(l)
    lua_close(l);

    result
  };

  RESULT.with(|r| cache_result(r, result))
}

#[cfg(test)]
mod tests {
  use core::{ffi::CStr, ptr::null};

  /// 成功脚本返回 null（结果为空）。
  #[test]
  fn clean_source_returns_null() {
    // SAFETY: 字面量 C 字符串。
    unsafe {
      assert!(super::execute_script(c"local x = 1".as_ptr()).is_null());
    }
  }

  /// 错误脚本返回缓存的错误信息 C 指针。
  #[test]
  fn error_source_returns_cached_message() {
    // SAFETY: 字面量 C 字符串。
    unsafe {
      let ptr = super::execute_script(c"error('boom')".as_ptr());
      assert!(!ptr.is_null());
      let msg = CStr::from_ptr(ptr).to_string_lossy();
      assert!(msg.contains("boom"), "got: {msg}");
    }
  }

  /// null 输入按空脚本处理，返回 null。
  #[test]
  fn null_source_returns_null() {
    // SAFETY: null 是契约允许的输入。
    unsafe {
      assert!(super::execute_script(null()).is_null());
    }
  }
}
