use core::slice::from_raw_parts_mut;

use crate::{
  enums::{t_key_view::TKeyView, value_view::ValueView},
  functions::countint::countint,
  macros::sizenode::sizenode,
  records::lua_table::LuaTable,
};

/// cpp `numusehash`（ltable.cpp）：统计哈希部分已用节点，整数键的区间计数写回
/// `nums`。
///
/// cpp 用 `int* pnasize` 出参累加数组部分增量并以返回值为已用节点数，Rust 版
/// 折叠为 `(ause, totaluse)` 元组返回。
///
/// # Safety
///
/// `t` 必须指向存活 `LuaTable`，其 `node` 指向 `sizenode(t)` 个可读 LuaNode；`nums` 为
/// 调用方保活的整数键区间计数数组。
pub(crate) unsafe fn numusehash(t: *const LuaTable, nums: &mut [i32]) -> (i32, i32) {
  let mut totaluse: i32 = 0; // total number of elements
  let mut ause: i32 = 0; // summation of `nums'
  // Safety: 契约保证 t 存活，sizenode 仅读其 node/sizearray 字段
  let sizenode = unsafe { sizenode!(t) as usize };

  // cpp `for (i = sizenode(t); i--;)` 逆序遍历哈希数组；Rust 版以可变切片 +
  // iter().rev() 等价展开，边界由契约一次锁定，循环体内不再逐格裸指针偏移
  // （本函数为纯统计：键/值槽位一律经 B2a TKeyView / B1 ValueView 只读视图读取，
  // 不写回任何节点字段）
  // Safety: 契约保证 node 指向 sizenode 个可读 LuaNode，切片构造不越界
  let nodes = unsafe { from_raw_parts_mut((*t).node, sizenode) };
  for n in nodes.iter().rev() {
    // 视图经共享引用读取，本循环体已无裸指针解引用
    if !matches!(ValueView::from_tvalue(&n.val), ValueView::Nil) {
      // 键轴（B2a TKeyView）：数值键候选链收敛为变体 match
      if let TKeyView::Number(key) = TKeyView::from_tkey(&n.key) {
        ause += countint(key, nums);
      }
      totaluse += 1;
    }
  }

  (ause, totaluse)
}
