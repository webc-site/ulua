//! 执行回调表（JIT/code-gen 挂点）。
//!
//! 回调签名保持 `extern "C-unwind"` 函数指针形态：ulua-code-gen 的 JIT 与
//! conformance 测试以同一 ABI 约定安装/调用它们（unwind 跨回调帧是显式契约），
//! 因此本表不是业务逻辑指针，而是 crate 内部的稳定回调 ABI，非 §2 待消灭对象。
//! 各槽位 `None` 表示该回调未安装（cpp 的 NULL 槽），空槽判定一律用 `Option`。

use core::ffi::{c_char, c_void};

use crate::records::{closure::Closure, lua_state::LuaState, proto::Proto};

#[repr(C)]
#[derive(Debug)]
pub struct lua_ExecutionCallbacks {
  /// 回调宿主数据；由安装方（code-gen/conformance）持有生命周期，`None` 语义不存在，
  /// 未安装回调时恒为 null 且不会被解引用。
  pub context: *mut c_void,
  pub close: Option<unsafe extern "C-unwind" fn(l: *mut LuaState)>,
  pub destroy: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto)>,
  pub enter: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto) -> i32>,
  pub disable: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto)>,
  pub getmemorysize:
    Option<unsafe extern "C-unwind" fn(l: *mut LuaState, proto: *mut Proto) -> usize>,
  pub gettypemapping:
    Option<unsafe extern "C-unwind" fn(l: *mut LuaState, str: *const c_char, len: usize) -> u8>,
  pub getcounterdata: Option<
    unsafe extern "C-unwind" fn(
      l: *mut LuaState,
      proto: *mut Proto,
      count: *mut usize,
    ) -> *mut c_char,
  >,
  pub inlinefunction: Option<
    unsafe extern "C-unwind" fn(
      l: *mut LuaState,
      caller: *mut Closure,
      target: *mut Closure,
      pc: u32,
    ) -> *mut Proto,
  >,
}

/// Rust 惯用名。原名 `lua_ExecutionCallbacks` 必须保留：`ulua-code-gen` 直接按该名字构造
/// 零值回调表（`repr(C)` 使混合大小写命名豁免 casing lint，改动会波及跨 crate 消费方）。
pub type LuaExecutionCallbacks = lua_ExecutionCallbacks;
