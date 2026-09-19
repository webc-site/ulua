//! 共享辅助: run_code / run_file 复用 cpp Repl.cpp 中重复两处的
//! "yield 消息或栈顶错误字符串 + trace 头 + debugtrace" 组装逻辑
//! （runCode 用 "\nstack backtrace:\n"，runFile 用 "\nstacktrace:\n"，头由调用方传入）。

use alloc::string::String;
use core::ffi::CStr;

use ulua_vm::{
  enums::lua_status::LuaStatus, functions::lua_debugtrace::lua_debugtrace,
  macros::lua_tostring::lua_tostring, type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// `state` 须为 resume 失败的有效 VM 状态，栈顶为错误对象。
pub unsafe fn error_with_trace(state: *mut lua_State, status: i32, trace_header: &str) -> String {
  let mut error = unsafe {
    if status == LuaStatus::Yield as i32 {
      String::from("thread yielded unexpectedly")
    } else {
      let str_ptr = lua_tostring!(state, -1);
      if str_ptr.is_null() {
        String::new()
      } else {
        // SAFETY: lua_tostring 返回 NUL 结尾字符串
        CStr::from_ptr(str_ptr).to_string_lossy().into_owned()
      }
    }
  };

  error.push_str(trace_header);
  // SAFETY: state 为有效 VM 状态
  let trace = unsafe { lua_debugtrace(state) };
  if !trace.is_null() {
    // SAFETY: lua_debugtrace 返回 NUL 结尾字符串
    error.push_str(&unsafe { CStr::from_ptr(trace) }.to_string_lossy());
  }

  // 错误对象的弹栈由调用方负责（run_code 弹 l 上的 thread，runFile 不弹）
  error
}
