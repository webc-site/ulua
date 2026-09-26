use core::ffi::c_void;

use crate::{
  enums::t_key_view::TKeyView,
  functions::{hashpointer::hashpointer, lua_a_toobject::LUA_O_NILOBJECT, walk_nodes::walk_nodes},
  records::lua_table::LuaTable,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `t` 须指向存活 `LuaTable` 且 `node` 区与 `sizenode` 一致（cpp ltable.cpp:1101）：
/// `hashpointer` 定位后沿 next 链游走仅限本表节点数组界内；`key/tag` 为值型比较，无内存前提。
pub unsafe fn lua_h_getp(t: *mut LuaTable, key: *mut c_void, tag: i32) -> *const TValue {
  unsafe {
    walk_nodes(
      hashpointer(t as *const _, key),
      |n| -> Option<*const TValue> {
        // 键轴读链收敛为 B2a TKeyView match：pointer+tag 两段比较与原
        // `ttislightuserdata!(nk) && pvalue!(nk) == key && lightuserdatatag!(nk) == tag` 逐位等价
        if matches!(TKeyView::from_tkey(&(*n).key),
        TKeyView::LightUserdata { pointer, tag: ktag }
          if pointer == key && ktag == tag)
        {
          return Some(&(*n).val);
        }
        None
      },
    )
    .unwrap_or(LUA_O_NILOBJECT)
  }
}
