use core::ffi::{c_char, c_uint};
#[derive(Debug)]
#[repr(C)]
pub struct LuauBuffer {
  pub(crate) tt: u8,
  pub(crate) marked: u8,
  pub(crate) memcat: u8,
  pub(crate) len: c_uint,
  pub(crate) _align: [u64; 0],
  pub(crate) data: [c_char; 1],
}

pub type Buffer = LuauBuffer;
