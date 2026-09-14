use core::{ffi::c_int, ptr::null_mut};

use crate::records::t_string::tstring;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LocVar {
  pub varname: *mut tstring,
  pub startpc: c_int,
  pub endpc: c_int,
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
