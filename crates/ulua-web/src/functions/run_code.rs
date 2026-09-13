//! `static std::string runCode(lua_State* l, const std::string& source)`
//! (`CLI/src/Web.cpp:71-140`).
//!
//! Compiles `source`, loads it into `l`, runs it on a fresh thread, prints any
//! results, and returns "" on success or a formatted error (with source:line
//! prefix and stack backtrace) on failure.

use alloc::string::String;
use core::{ffi::c_void, mem, ptr::null_mut};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_debugtrace::lua_debugtrace, lua_getinfo::lua_getinfo, lua_gettop::lua_gettop,
    lua_insert::lua_insert, lua_l_checkstack::lua_l_checkstack, lua_newthread::lua_newthread,
    lua_pcall::lua_pcall, lua_pushvalue::lua_pushvalue, lua_remove::lua_remove,
    lua_resume::lua_resume, lua_tolstring::lua_tolstring, lua_xmove::lua_xmove,
    luau_load::luau_load,
  },
  macros::{lua_getglobal::lua_getglobal, lua_minstack::LUA_MINSTACK, lua_pop::lua_pop},
  records::lua_debug::LuaDebug,
  type_aliases::lua_state::lua_State,
};

use crate::util::cstr_cow;

// 与 `luau_compile` 内部的 `malloc` 配对：字节缓冲区按 C API 约定由调用方用
// libc `free` 释放。真 C ABI，保留薄壳。
unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

/// # Safety
/// `l` must be a valid, non-null pointer to an initialized `lua_State`.
pub unsafe fn run_code(l: *mut lua_State, source: &str) -> String {
  unsafe {
    // size_t bytecodeSize = 0;
    // char* bytecode = luau_compile(source.data(), source.length(), nullptr, &bytecodeSize);
    let mut bytecode_size: usize = 0;
    let bytecode = luau_compile(
      source.as_ptr().cast(),
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );

    // int result = luau_load(l, "=stdin", bytecode, bytecodeSize, 0);
    let result = luau_load(l, c"=stdin".as_ptr(), bytecode, bytecode_size, 0);

    // free(bytecode);
    free(bytecode.cast());

    if result != 0 {
      // 加载失败：错误信息在栈顶（读取后弹出）。
      let mut len: usize = 0;
      let error = cstr_cow(lua_tolstring(l, -1, &mut len)).into_owned();
      lua_pop(l, 1);
      return error;
    }

    // lua_State* T = lua_newthread(l);
    let t = lua_newthread(l);

    // lua_pushvalue(l, -2);
    // lua_remove(l, -3);
    // lua_xmove(l, T, 1);
    lua_pushvalue(l, -2);
    lua_remove(l, -3);
    lua_xmove(l, t, 1);

    // int status = lua_resume(T, NULL, 0);
    let status = lua_resume(t, null_mut(), 0);

    if status == LuaStatus::Ok as i32 {
      let n = lua_gettop(t);

      if n != 0 {
        lua_l_checkstack(t, LUA_MINSTACK, "too many results to print");
        lua_getglobal(t, c"print".as_ptr());
        lua_insert(t, 1);
        lua_pcall(t, n, 0, 0);
      }

      lua_pop(l, 1); // pop T
      String::new()
    } else {
      let mut error = String::new();

      // LuaDebug ar;
      // if (lua_getinfo(l, 0, "sln", &ar))
      // LuaDebug 是纯 POD（指针/整数/数组），全零即全 null/0，合法初值。
      let mut ar: LuaDebug = mem::zeroed();
      if lua_getinfo(l, 0, c"sln".as_ptr(), &mut ar) != 0 {
        error.push_str(&cstr_cow(ar.short_src));
        error.push(':');
        error.push_str(&ar.currentline.to_string());
        error.push_str(": ");
      }

      if status == LuaStatus::Yield as i32 {
        error.push_str("thread yielded unexpectedly");
      } else {
        // else if (const char* str = lua_tostring(T, -1))
        error.push_str(&cstr_cow(lua_tolstring(t, -1, null_mut())));
      }

      error.push_str("\nstack backtrace:\n");
      error.push_str(&cstr_cow(lua_debugtrace(t)));

      lua_pop(l, 1); // pop T
      error
    }
  }
}

#[cfg(test)]
mod tests {
  use ulua_vm::functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate};

  use super::run_code;
  use crate::functions::setup_state::setup_state;

  /// 成功运行返回空串。
  #[test]
  fn ok_run_returns_empty() {
    let l = lua_l_newstate();
    // SAFETY: l 是刚创建的合法 lua_State。
    unsafe {
      setup_state(l);
      let result = run_code(l, "print('hi')");
      assert_eq!(result, "");
      lua_close(l);
    }
  }

  /// 运行期错误：返回错误信息 + 栈回溯。
  #[test]
  fn runtime_error_has_message_and_backtrace() {
    let l = lua_l_newstate();
    // SAFETY: l 是刚创建的合法 lua_State。
    unsafe {
      setup_state(l);
      let result = run_code(l, "error('boom')");
      assert!(result.contains("boom"), "got: {result}");
      assert!(result.contains("stack backtrace:"), "got: {result}");
      lua_close(l);
    }
  }

  /// 编译错误：luau_load 失败，返回错误信息。
  #[test]
  fn compile_error_is_reported() {
    let l = lua_l_newstate();
    // SAFETY: l 是刚创建的合法 lua_State。
    unsafe {
      setup_state(l);
      let result = run_code(l, "local x =");
      assert!(!result.is_empty(), "got: {result}");
      assert!(result.contains(":1:"), "got: {result}");
      lua_close(l);
    }
  }

  /// 多返回值经 print 打印，run_code 仍返回空串。
  #[test]
  fn multiple_results_are_printed() {
    let l = lua_l_newstate();
    // SAFETY: l 是刚创建的合法 lua_State。
    unsafe {
      setup_state(l);
      let result = run_code(l, "return 1, 2");
      assert_eq!(result, "");
      lua_close(l);
    }
  }
}
