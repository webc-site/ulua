//! `extern "C" const char* executeScript(const char* source)`
//! (`CLI/src/Web.cpp:184-208`).
//!
//! The wasm entry point: enables every `Luau*` bool fast flag, spins up a fresh
//! sandboxed Lua state, runs the script via [`run_code`], and returns the result
//! string (or null when empty). The C++ caches the result in a function-`static`
//! `std::string` so the returned pointer outlives the call; the Rust analog is a
//! thread-local `CString`.

use core::{
  cell::RefCell,
  ffi::{CStr, c_char},
  str,
};
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
  util::cache_result,
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
  unsafe {
    // setup flags:
    //   for (FValue<bool>* flag = FValue<bool>::list; flag; flag = flag->next)
    //       if (strncmp(flag->name, "Luau", 4) == 0)
    //           flag->value = true;
    //
    // In this port the per-type intrusive `FValue<bool>::list` is never
    // populated (Rust statics cannot self-register in a ctor, and no startup
    // `register()` runs), so the C++ name-prefix walk is expressed through the
    // crate's public flag-enabling analog `set_luau_bool_flags`, which turns on
    // every non-`Debug` bool FastFlag (the `Luau*` flags dominate that set).
    set_luau_bool_flags(true);

    // create new state: unique_ptr<lua_State, lua_close> globalState(luaL_newstate(), lua_close);
    let l: *mut lua_State = lua_l_newstate();

    // setup state
    setup_state(l);

    // sandbox thread
    lua_l_sandboxthread(l);

    // run code + collect error
    let source_str = if source.is_null() {
      ""
    } else {
      str::from_utf8_unchecked(CStr::from_ptr(source).to_bytes())
    };
    let result = run_code(l, source_str);

    // unique_ptr destructor: lua_close(l)
    lua_close(l);

    RESULT.with(|r| cache_result(r, result))
  }
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
