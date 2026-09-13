use core::ffi::{c_char, c_uint};

use crate::records::g_cheader::GCheader;
#[derive(Debug)]
#[repr(C)]
pub struct tstring {
  pub(crate) hdr: GCheader,
  pub(crate) _padding1: [c_char; 1],
  pub(crate) atom: i16,
  pub(crate) _padding2: [c_char; 2],
  pub(crate) next: *mut tstring,
  pub(crate) hash: c_uint,
  pub(crate) len: c_uint,
  pub(crate) data: [c_char; 1],
}
