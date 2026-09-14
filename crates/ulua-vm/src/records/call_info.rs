use core::{
  ffi::{c_int, c_uint},
  ptr::{null, null_mut},
};

use crate::type_aliases::stk_id::StkId;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CallInfo {
  pub base: StkId,
  pub func: StkId,
  pub top: StkId,
  pub savedpc: *const u32,
  pub errfunc: c_int,
  pub nresults: c_int,
  pub flags: c_uint,
}

impl Default for CallInfo {
  fn default() -> Self {
    Self {
      base: null_mut(),
      func: null_mut(),
      top: null_mut(),
      savedpc: null(),
      errfunc: 0,
      nresults: 0,
      flags: 0,
    }
  }
}
