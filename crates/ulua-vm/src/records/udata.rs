use core::ffi::{c_char, c_int};

use crate::records::lua_table::LuaTable;
#[repr(C)]
#[derive(Debug)]
pub struct Udata {
  pub(crate) tt: u8,
  pub(crate) marked: u8,
  pub(crate) memcat: u8,

  pub tag: u8,

  pub len: c_int,

  pub metatable: *mut LuaTable,

  pub(crate) _align: [u64; 0],
  pub data: [c_char; 1],
}
