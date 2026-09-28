use core::fmt::{Debug, Formatter, Result};

use crate::{records::g_cheader::GCheader, type_aliases::t_value::TValue};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UpValOpen {
  pub prev: *mut UpVal,
  pub next: *mut UpVal,
  pub threadnext: *mut UpVal,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UpValInner {
  pub value: TValue,
  pub open: UpValOpen,
}

impl Debug for UpValInner {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("UpValInner").finish_non_exhaustive()
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UpVal {
  pub hdr: GCheader,
  pub markedopen: u8,
  pub _padding: [u8; 4],
  pub v: *mut TValue,
  pub u: UpValInner,
}
