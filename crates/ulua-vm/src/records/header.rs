use core::{ffi::c_int, ptr::null_mut};

use crate::records::lua_state::LuaState;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Header {
  pub(crate) l: *mut LuaState,
  pub(crate) islittle: c_int,
  pub(crate) maxalign: c_int,
}

impl Default for Header {
  fn default() -> Self {
    Self {
      l: null_mut(),
      islittle: 0,
      maxalign: 0,
    }
  }
}
