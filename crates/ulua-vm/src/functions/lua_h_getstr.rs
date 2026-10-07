//! Source: `VM/src/ltable.cpp` (ltable.cpp:657-669, hand-ported)

use core::ptr::eq;

use crate::{
  functions::walk_nodes::walk_nodes,
  macros::{gkey::gval, hashstr::hashstr},
  records::{lua_table::LuaTable, slot::Slot, t_string::tstring},
};

/// # Safety
/// `t` 须为存活 `LuaTable` 的共享只读借用且 `node` 区与 `sizenode` 一致，`key` 须为存活 interned
/// `tstring`（cpp ltable.cpp:1084）：`hashstr!` 定位后沿 next 链游走仅限本表节点数组界内。
///
/// B2-2a 任务B试点（表槽返回族句柄化形态）：miss 由 cpp 全局哨兵 `luaO_nilobject` 折叠为
/// `None`；命中槽以 `Slot<'a>` 返回、生命周期锚定 `&'a LuaTable` 借用——表结构性变更
/// （rehash/扩数组会整体移动节点槽）不受生命周期约束，句柄有效性沿用既有契约：仅在持有
/// 该表借用、且其间无 newkey/rehash 介入的窗口内读写；跨调用持有槽地址的消费链
/// （gval2slot/cachedslot、指针出参传播）在边界经 `as_const_ptr` 还原裸形。
#[inline]
pub unsafe fn lua_h_getstr<'a>(t: &'a LuaTable, key: *mut tstring) -> Option<Slot<'a>> {
  unsafe {
    walk_nodes(hashstr!(t, key), |n| -> Option<Slot<'a>> {
      // 对照 cpp ltable.cpp:1089: if (ttisstring(gkey(n)) && tsvalue(gkey(n)) == key) return gval(n);
      if (*n).key.is_string() && eq((*n).key.as_string_ptr(), key) {
        return Some(Slot::from_ref(&*gval!(n))); // that's it
      }
      None
    })
  }
}
