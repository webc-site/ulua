use core::fmt::{Debug, Formatter, Result};

use crate::records::{gc_object::GcObject, lua_node::LuaNode, lua_t_value::TValue};
#[repr(C)]
#[derive(Copy, Clone)]
pub union LuaTable_Union {
  pub lastfree: i32,
  pub aboundary: i32,
}

impl Debug for LuaTable_Union {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("LuaTable_Union").finish_non_exhaustive()
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LuaTable {
  pub tt: u8,
  pub marked: u8,
  pub memcat: u8,

  pub tmcache: u8,
  pub readonly: u8,
  pub safeenv: u8,
  pub lsizenode: u8,
  pub nodemask8: u8,

  pub sizearray: i32,
  pub union: LuaTable_Union,

  pub metatable: *mut LuaTable,
  pub array: *mut TValue,
  pub node: *mut LuaNode,
  pub gclist: *mut GcObject,
}

impl LuaTable {}
