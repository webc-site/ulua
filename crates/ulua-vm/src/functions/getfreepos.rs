use core::ptr::NonNull;

use crate::{
  enums::t_key_view::TKeyView,
  macros::gnode::gnode,
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// # Safety
/// `t` 须为存活 `LuaTable` 且其哈希部分已分配（`gnode!(t, i)` 依赖 `sizenode(t)>0`），
/// `(*t).union.lastfree` 处于 `[0, sizenode(t)]` 内，循行为其递减并经 `TKeyView::Nil`
/// （B2a 键轴读链）判 `gkey` 的空槽标记。
/// 返回 `Some(可写空闲 LuaNode)`，或 `None` 表示「无空槽」（cpp `return NULL`）。
/// cpp `ltable.cpp:1006`。
///
/// 形态选择（review.md §2「可空指针 → Option」在本处的等价 Rust 落形）：返回
/// `Option<NonNull<LuaNode>>` 而非 `Option<&mut LuaNode>` —— 唯一消费方 `newkey` 的
/// 后续写路径依赖节点裸指针身份（`mp`/`othern` 与 `n` 的 `offset_from` 链改写、
/// `eq(n, dummynode)` 指针相等判据、`mp = n` 重指派），且 `mainposition` 契约明载
/// 「provenance 须挂在表裸指针下，不得降为 `&LuaTable`」。若在此物化 `&mut LuaNode`，
/// 它会与同函数内对同一段实向量的兄弟裸写（`(*othern).key.set_next`、
/// `setnilvalue!(gval!(mp))`）互相别名，且需为 `*mut LuaTable` 派生的借用伪造寿命。
/// `Option<NonNull<_>>` 把可空性收进类型（null 哨兵归零、 niche 优化零开销），不改
/// 动 provenance 形状，是本借用布局下的最小安全形态。
pub(crate) unsafe fn getfreepos(t: *mut LuaTable) -> Option<NonNull<LuaNode>> {
  unsafe {
    // In the C++ source, lastfree is accessed as t->lastfree.
    // In the Rust LuaTable record, lastfree is part of the union.
    // We access it through the union field.
    while (*t).union.lastfree > 0 {
      (*t).union.lastfree -= 1;

      let n = gnode!(t, (*t).union.lastfree);
      if matches!(TKeyView::from_tkey(&(*n).key), TKeyView::Nil) {
        return NonNull::new(n);
      }
    }
    None // could not find a free place
  }
}
