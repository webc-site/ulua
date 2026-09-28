use core::ptr::{null, null_mut};

use crate::type_aliases::stk_id::StkId;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CallInfo {
  pub base: StkId,
  pub func: StkId,
  pub top: StkId,
  pub savedpc: *const u32,
  pub errfunc: i32,
  pub nresults: i32,
  pub flags: u32,
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
