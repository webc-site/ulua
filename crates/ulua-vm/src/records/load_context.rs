use core::ffi::c_char;

use crate::records::{proto::Proto, t_string::tstring, temp_buffer::TempBuffer};

#[repr(C)]
#[derive(Debug)]
pub struct LoadContext {
  pub(crate) strings: TempBuffer<*mut tstring>,
  pub(crate) protos: TempBuffer<*mut Proto>,
  pub(crate) chunkname: *const c_char,
  pub(crate) data: *const c_char,
  pub(crate) size: usize,
  pub(crate) env: i32,
  pub(crate) result: i32,
}
