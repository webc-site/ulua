use core::{ffi::c_int, ptr::null_mut};

use crate::records::t_string::tstring;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Stringtable {
  pub(crate) hash: *mut *mut tstring,
  pub(crate) nuse: u32,
  pub(crate) size: c_int,
}

impl Default for Stringtable {
  fn default() -> Self {
    Self {
      hash: null_mut(),
      nuse: 0,
      size: 0,
    }
  }
}

pub use Stringtable as stringtable;
