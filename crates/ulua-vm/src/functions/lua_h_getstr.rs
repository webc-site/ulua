//! Source: `VM/src/ltable.cpp` (ltable.cpp:657-669, hand-ported)

use core::ptr::eq;

use crate::{
  functions::walk_nodes::walk_nodes,
  macros::{gkey::gval, hashstr::hashstr, lua_o_nilobject::LUA_O_NILOBJECT},
  records::{lua_table::LuaTable, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t` 须指向存活 `LuaTable` 且 `node` 区与 `sizenode` 一致，`key` 须为存活 interned
/// `tstring`（cpp ltable.cpp:1084）：`hashstr!` 定位后沿 next 链游走仅限本表节点数组界内。
pub unsafe fn lua_h_getstr(t: *mut LuaTable, key: *mut tstring) -> *const TValue {
  unsafe {
    walk_nodes(hashstr!(t, key), |n| -> Option<*const TValue> {
      // 对照 cpp ltable.cpp:1089: if (ttisstring(gkey(n)) && tsvalue(gkey(n)) == key) return gval(n);
      if (*n).key.is_string() && eq((*n).key.as_string_ptr(), key) {
        return Some(gval!(n)); // that's it
      }
      None
    })
    .unwrap_or(LUA_O_NILOBJECT)
  }
}
