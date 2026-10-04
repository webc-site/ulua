//! 共享辅助: run_loaded_chunk / runFile 复用 cpp Repl.cpp 中重复两处的
//! "yield 消息或栈顶错误字符串 + trace 头 + debugtrace" 组装逻辑
//! （runCode 用 "\nstack backtrace:\n"，runFile 用 "\nstacktrace:\n"，头由调用方传入）。
//! 原实现散落在 ulua-repl-cli（error_with_trace）与 ulua（stack_string）两处，
//! 现收口进 VM 供三方薄壳共用。

use alloc::string::String;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_debugtrace::lua_debugtrace, lua_tolstring::lua_tolstring_ref},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `state` 须为 resume 失败的有效 VM 状态，栈顶为错误对象。
pub unsafe fn error_with_trace(state: *mut LuaState, status: i32, trace_header: &str) -> String {
  let mut error = if status == LuaStatus::Yield as i32 {
    String::from("thread yielded unexpectedly")
  } else {
    // 错误文本按全字节读取（lua_tolstring_ref 含数值/__tostring 转换，嵌入 NUL
    // 不截断）——DELIBERATE DEVIATION：cpp `lua_tostring` 走 C 串在首 NUL 处截断，
    // 本仓三调用方原语义中 ulua/web 均为字节保真（超集偏差已锚定），repl 随之统一。
    // 非串错误对象得 None，译成空串后仅携带回溯，与 cpp 分支观察一致。
    // SAFETY: state 存活且 -1 为错误对象槽；返回切片指向该栈槽串内容。
    unsafe { lua_tolstring_ref(state, -1) }
      .map_or_else(String::new, |s| String::from_utf8_lossy(s).into_owned())
  };

  error.push_str(trace_header);
  // SAFETY: state 为有效 VM 状态；§10 后 `lua_debugtrace` 直接返回回溯字节窗
  //（旧指针面按首 NUL 截读，窗内各帧字段写端已同点截断，lossy 渲染逐字节不变）
  error.push_str(&String::from_utf8_lossy(unsafe { lua_debugtrace(state) }));

  // 错误对象的弹栈由调用方负责（run_loaded_chunk 弹 l 上的 thread，runFile 不弹）
  error
}
