use core::ffi::c_int;

use crate::records::{gc_object::GcObject, lua_t_value::TValue, luau_class::LuauClass};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LuauObject {
  pub(crate) tt: u8,
  pub(crate) marked: u8,
  pub(crate) memcat: u8,

  pub gclist: *mut GcObject,

  /// The class object that this value is an instance of.
  pub lclass: *mut LuauClass,

  /// The number of members that this instance contains. We need this in order
  /// to free ourselves if we got swept in the same GC cycle as our class
  /// pointer.
  pub numberofmembers: c_int,

  /// The fields of this instance.
  pub members: *mut TValue,
}

impl LuauObject {}
