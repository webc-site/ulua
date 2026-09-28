use core::ptr::null_mut;

use crate::records::t_string::tstring;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LocVar {
  pub varname: *mut tstring,
  pub startpc: i32,
  pub endpc: i32,
  pub reg: u8,
}

impl Default for LocVar {
  fn default() -> Self {
    Self {
      varname: null_mut(),
      startpc: 0,
      endpc: 0,
      reg: 0,
    }
  }
}
