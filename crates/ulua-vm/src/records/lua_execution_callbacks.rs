use core::ffi::{c_char, c_int, c_void};

use crate::records::{closure::Closure, lua_state::lua_State, proto::Proto};

#[repr(C)]
#[derive(Debug)]
pub struct lua_ExecutionCallbacks {
  pub context: *mut c_void,
  pub close: Option<unsafe extern "C-unwind" fn(l: *mut lua_State)>,
  pub destroy: Option<unsafe extern "C-unwind" fn(l: *mut lua_State, proto: *mut Proto)>,
  pub enter: Option<unsafe extern "C-unwind" fn(l: *mut lua_State, proto: *mut Proto) -> c_int>,
  pub disable: Option<unsafe extern "C-unwind" fn(l: *mut lua_State, proto: *mut Proto)>,
  pub getmemorysize:
    Option<unsafe extern "C-unwind" fn(l: *mut lua_State, proto: *mut Proto) -> usize>,
  pub gettypemapping:
    Option<unsafe extern "C-unwind" fn(l: *mut lua_State, str: *const c_char, len: usize) -> u8>,
  pub getcounterdata: Option<
    unsafe extern "C-unwind" fn(
      l: *mut lua_State,
      proto: *mut Proto,
      count: *mut usize,
    ) -> *mut c_char,
  >,
  pub inlinefunction: Option<
    unsafe extern "C-unwind" fn(
      l: *mut lua_State,
      caller: *mut Closure,
      target: *mut Closure,
      pc: u32,
    ) -> *mut Proto,
  >,
}

pub type LuaExecutionCallbacks = lua_ExecutionCallbacks;
