use core::ptr::null_mut;

use crate::{
  enums::t_key_view::TKeyView,
  macros::gnode::gnode,
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// # Safety
/// `t` 须为存活 `LuaTable` 且其哈希部分已分配（`gnode!(t, i)` 依赖 `sizenode(t)>0`），
/// `(*t).union.lastfree` 处于 `[0, sizenode(t)]` 内，循行为其递减并经 `TKeyView::Nil`
/// （B2a 键轴读链）判 `gkey` 的空槽标记。
/// 返回可写的空闲 `LuaNode` 或 `null_mut()`。cpp `ltable.cpp:869`。
pub(crate) unsafe fn getfreepos(t: *mut LuaTable) -> *mut LuaNode {
  unsafe {
    // In the C++ source, lastfree is accessed as t->lastfree.
    // In the Rust LuaTable record, lastfree is part of the union.
    // We access it through the union field.
    while (*t).union.lastfree > 0 {
      (*t).union.lastfree -= 1;

      let n = gnode!(t, (*t).union.lastfree);
      if matches!(TKeyView::from_tkey(&(*n).key), TKeyView::Nil) {
        return n;
      }
    }
    null_mut() // could not find a free place
  }
}
