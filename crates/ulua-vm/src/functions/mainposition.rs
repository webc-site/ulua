use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{hashint::hashint, hashnum::hashnum, hashpointer::hashpointer, hashvec::hashvec},
  macros::{
    gcvalue::gcvalue, hashboolean::hashboolean, hashstr::hashstr, lvalue::lvalue, pvalue::pvalue,
    ttype::ttype,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable, t_key::TKey},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t` 须为存活 `LuaTable` 的裸指针（返回值是可写节点别名，provenance 须挂在表裸指针下，
/// 不得降为 `&LuaTable`）；`key` 须为存活 `TValue` 的共享只读借用——本函数只读 `key` 的
/// tag/payload 后转交各 hash 函数，体内无任何分配或元方法调用。传入的指针必须有效且指向
/// 存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn mainposition(t: *const LuaTable, key: &TValue) -> *mut LuaNode {
  unsafe {
    match ttype!(key) {
      x if x == LuaType::Number as u32 => hashnum(&*t, key.as_number()),
      x if x == LuaType::String as u32 => hashstr!(t, key.as_string()),
      x if x == LuaType::Integer as u32 => hashint(&*t, lvalue!(key)),
      x if x == LuaType::Vector as u32 => hashvec(&*t, key.as_vector_ref()),
      x if x == LuaType::Boolean as u32 => hashboolean!(t, key.as_boolean_raw()),
      x if x == LuaType::LightUserData as u32 => hashpointer(t, pvalue!(key)),
      _ => hashpointer(t, gcvalue!(key) as *const c_void),
    }
  }
}

/// 直接从 [`TKey`] 计算主位，消除构造 16 字节中间栈对象与 checkliveness 的额外开销。
///
/// # Safety
/// `t` 须为存活 `LuaTable` 裸指针；`key` 须为存活 `TKey` 的共享只读借用。
#[inline(always)]
pub(crate) unsafe fn mainposition_tkey(t: *const LuaTable, key: &TKey) -> *mut LuaNode {
  unsafe {
    match key.tt() {
      x if x == LuaType::Number as i32 => hashnum(&*t, key.as_number()),
      x if x == LuaType::String as i32 => hashstr!(t, key.as_string()),
      x if x == LuaType::Integer as i32 => hashint(&*t, lvalue!(key)),
      x if x == LuaType::Vector as i32 => hashvec(&*t, key.as_vector_ref()),
      x if x == LuaType::Boolean as i32 => hashboolean!(t, key.as_boolean_raw()),
      x if x == LuaType::LightUserData as i32 => hashpointer(t, pvalue!(key)),
      _ => hashpointer(t, gcvalue!(key) as *const c_void),
    }
  }
}
