use core::ffi::c_int;

use crate::records::{
  gc_object::GcObject, lua_t_value::TValue, lua_table::LuaTable, t_string::tstring,
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LuauClass {
  pub(crate) tt: u8,
  pub(crate) marked: u8,
  pub(crate) memcat: u8,

  pub gclist: *mut GcObject,

  pub name: *mut tstring,

  pub super_: *mut LuauClass,

  pub staticmembers: *mut TValue,

  pub memberstooffset: *mut LuaTable,

  pub offsettomember: *mut *mut tstring,

  pub instancemetatable: *mut LuaTable,

  pub numberofinstancemembers: c_int,

  pub numberofallmembers: c_int,

  pub isopen: bool,

  pub hasuserinitinchain: bool,
}
