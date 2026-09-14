use core::{
  ffi::{c_char, c_int},
  ptr::{null, null_mut},
};

use crate::records::lua_state::LuaState;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MatchState {
  pub(crate) matchdepth: c_int,
  pub(crate) src_init: *const c_char,
  pub(crate) src_end: *const c_char,
  pub(crate) p_end: *const c_char,
  pub(crate) l: *mut LuaState,
  pub(crate) level: c_int,
  pub(crate) capture: [MatchState_capture; 32],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MatchState_capture {
  pub(crate) init: *const c_char,
  pub(crate) len: isize,
}

impl Default for MatchState {
  fn default() -> Self {
    Self {
      matchdepth: 0,
      src_init: null(),
      src_end: null(),
      p_end: null(),
      l: null_mut(),
      level: 0,
      capture: [MatchState_capture {
        init: null(),
        len: 0,
      }; 32],
    }
  }
}
