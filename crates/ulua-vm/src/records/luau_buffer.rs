use core::ffi::{c_char, c_uint};
#[derive(Debug)]
#[repr(C)]
pub struct LuauBuffer {
  pub(crate) tt: u8,
  pub(crate) marked: u8,
  pub(crate) memcat: u8,
  // JIT codegen 以 offset_of!/offsetof 访问, 与 Udata 的公开字段同例
  pub len: c_uint,
  pub(crate) _align: [u64; 0],
  pub data: [c_char; 1],
}

pub type Buffer = LuauBuffer;
pub type LuaBuffer = LuauBuffer;
