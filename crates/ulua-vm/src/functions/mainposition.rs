use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{hashint::hashint, hashnum::hashnum, hashpointer::hashpointer, hashvec::hashvec},
  macros::{
    gcvalue::gcvalue, hashboolean::hashboolean, hashstr::hashstr, lvalue::lvalue, pvalue::pvalue,
    ttype::ttype,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn mainposition(t: *const LuaTable, key: *const TValue) -> *mut LuaNode {
  unsafe {
    match ttype!(key) {
      x if x == LuaType::Number as u32 => hashnum(t as *mut LuaTable, (*key).as_number()),
      x if x == LuaType::Integer as u32 => hashint(t, lvalue!(key)),
      x if x == LuaType::Vector as u32 => hashvec(t, (*key).as_vector_ref().as_ptr()),
      x if x == LuaType::String as u32 => hashstr!(t, (*key).as_string()),
      x if x == LuaType::Boolean as u32 => hashboolean!(t, (*key).as_boolean_raw()),
      x if x == LuaType::LightUserData as u32 => hashpointer(t, pvalue!(key)),
      _ => hashpointer(t, gcvalue!(key) as *const c_void),
    }
  }
}
