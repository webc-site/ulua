//! LuaNode next 链遍历共用助手：收拢 ltable 查找族（`lua_h_getstr`/`luaH_get`/
//! `luaH_getp`/`luaH_getnum`/`findindex`）逐位同构的「loop{命中返回; next 为 0
//! 截止; 否则前进}」手写循环。

use crate::records::lua_node::LuaNode;

/// 沿 `n` 起的节点链游走：每节点先调 `f`，得 `Some(r)` 立即返回；否则读
/// `key.next`（ltable.h 的 `gnext(n)`）单次，为 0 返回 `None`，非 0 前进。
/// 单态化后与各调用方原手写循环同机器码（`#[inline(always)]`，无间接调用）。
///
/// # Safety
/// `n` 须指向存活 `LuaNode`，且沿 next 链前进全程落在其所属表节点数组界内
/// （与调用方原手写循环同一前提）。
#[inline(always)]
pub(crate) unsafe fn walk_nodes<R>(
  mut n: *mut LuaNode,
  mut f: impl FnMut(*mut LuaNode) -> Option<R>,
) -> Option<R> {
  // Safety: 契约保证 `n` 及沿 next 链前进的每个节点均在所属表节点数组界内且存活
  unsafe {
    loop {
      if let Some(r) = f(n) {
        return Some(r);
      }
      let next = (*n).key.next();
      if next == 0 {
        return None;
      }
      n = n.offset(next as isize);
    }
  }
}
