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
  // JIT codegen 以 offset_of!/offsetof 访问, 与 Udata 的公开字段同例
  pub len: c_uint,
  pub(crate) data: [c_char; 1],
}

pub type TString = tstring;
