//! Source: `VM/include/lua.h` (lua.h:611-640, hand-ported)

use core::ffi::{c_char, c_void};

use crate::{records::lua_state::LuaState, type_aliases::lua_hook::LuaHook};

/// 仅携 `LuaState` 一参的回调签名：cpp lua.h 中 debugprotectederror/preresume/
/// postresume 三者的行内声明形状一致，收敛为一个别名（与 [`LuaHook`] 同款先例）。
type StateOnlyCallback = Option<unsafe extern "C-unwind" fn(l: *mut LuaState)>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LuaCallbacks {
  /// arbitrary userdata pointer that is never overwritten by Luau
  pub userdata: *mut c_void,

  /// gets called at safepoints (loop back edges, call/ret, gc) if set
  pub interrupt: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, gc: i32)>,
  /// gets called when panic happens
  pub panic: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, errcode: i32)>,

  /// gets called when a new thread is created
  pub userthread: Option<unsafe extern "C-unwind" fn(lp: *mut LuaState, l: *mut LuaState)>,
  /// gets called when a string is created to assign an atom id
  pub useratom:
    Option<unsafe extern "C-unwind" fn(l: *mut LuaState, s: *const c_char, len: usize) -> i16>,

  /// gets called when BREAK instruction is encountered（cpp lua.h lua_Hook 同签名）
  pub debugbreak: LuaHook,
  /// gets called after each instruction in single step mode（cpp lua.h lua_Hook 同签名）
  pub debugstep: LuaHook,
  /// gets called when thread execution is interrupted by break in another thread
  /// （cpp lua.h lua_Hook 同签名）
  pub debuginterrupt: LuaHook,
  /// gets called when protected call results in an error
  pub debugprotectederror: StateOnlyCallback,

  /// NOTE: experimental API, requires a Debug flag to be called and is subject to
  /// breaking changes（cpp lua.h:622）：finalizer 附着前回调
  pub userfinalizer: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, co: *mut LuaState)>,

  /// gets called after a heap object (or array) is allocated
  /// （cpp lua.h:630 全参：block/memcat/tt/tag 同供）
  pub onallocate: Option<
    unsafe extern "C-unwind" fn(
      l: *mut LuaState,
      block: *mut c_void,
      osize: usize,
      nsize: usize,
      memcat: u8,
      tt: i32,
      tag: i32,
    ),
  >,

  /// gets called before lua_resume runs a (co)routine（cpp lua.h:632）
  pub preresume: StateOnlyCallback,
  /// gets called after lua_resume returns (yield, return, or error)（cpp lua.h:633）
  pub postresume: StateOnlyCallback,

  /// gets called before a heap object (or array) is freed（cpp lua.h:636）
  pub onfree: Option<unsafe extern "C-unwind" fn(l: *mut LuaState, block: *mut c_void)>,
}
