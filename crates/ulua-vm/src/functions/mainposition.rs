use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{hashint::hashint, hashnum::hashnum, hashpointer::hashpointer, hashvec::hashvec},
  macros::{
    bvalue::bvalue, gcvalue::gcvalue, hashboolean::hashboolean, hashstr::hashstr, lvalue::lvalue,
    nvalue::nvalue, pvalue::pvalue, tsvalue::tsvalue, ttype::ttype, vvalue::vvalue,
  },
  type_aliases::{lua_node::LuaNode, lua_table::LuaTable, t_value::TValue},
};

pub(crate) unsafe fn mainposition(t: *const LuaTable, key: *const TValue) -> *mut LuaNode {
  unsafe {
    match ttype!(key) {
      x if x == LuaType::Number as i32 => hashnum(t as *mut LuaTable, nvalue!(key)),
      x if x == LuaType::Integer as i32 => hashint(t, lvalue!(key)),
      x if x == LuaType::Vector as i32 => hashvec(t, vvalue!(key).as_ptr()),
      x if x == LuaType::String as i32 => hashstr!(t, tsvalue!(key)),
      x if x == LuaType::Boolean as i32 => hashboolean!(t, bvalue!(key)),
      x if x == LuaType::LightUserData as i32 => hashpointer(t, pvalue!(key)),
      _ => hashpointer(t, gcvalue!(key) as *const c_void),
    }
  }
}
