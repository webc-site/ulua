use core::ffi::{c_char, c_void};

use crate::type_aliases::lua_state::lua_State;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct EnumContext {
  pub l: *mut lua_State,
  pub context: *mut c_void,
  pub node: Option<
    unsafe extern "C-unwind" fn(
      context: *mut c_void,
      ptr: *mut c_void,
      tt: u8,
      memcat: u8,
      size: usize,
      name: *const c_char,
    ),
  >,
  pub edge: Option<
    unsafe extern "C-unwind" fn(
      context: *mut c_void,
      from: *mut c_void,
      to: *mut c_void,
      name: *const c_char,
    ),
  >,
}
