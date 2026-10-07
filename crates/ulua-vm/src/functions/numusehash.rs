use crate::{
  enums::{t_key_view::TKeyView, value_view::ValueView},
  functions::countint::countint,
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
/// `t` 必须指向存活 `LuaTable`，其哈希段按 `node_window()` 共享窗可读（哨兵表窗长
/// 恒 1、实向量窗长 `sizenode(t)`，均须与元数据一致）；`nums` 为调用方保活的整数键
/// 区间计数数组。
pub(crate) unsafe fn numusehash(t: *const LuaTable, nums: &mut [i32]) -> (i32, i32) {
  let mut totaluse: i32 = 0; // total number of elements
  let mut ause: i32 = 0; // summation of `nums'

  // cpp `for (i = sizenode(t); i--;)` 逆序遍历哈希数组；Rust 版切 node_window 共享窗 +
  // iter().rev() 等价展开，窗形收口切片构造（免手工 from_raw_parts_mut 的哨兵 &mut
  // 别名违例），循环体内不再逐格裸指针偏移
  // （本函数为纯统计：键/值槽位一律经 B2a TKeyView / B1 ValueView 只读视图读取，
  // 不写回任何节点字段；哨兵表读出单格 dummy，val 恒 nil ⇒ 空桶不计数，与 cpp 一致）
  for n in unsafe { (*t).node_window() }.iter().rev() {
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
