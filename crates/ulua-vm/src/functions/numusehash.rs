use crate::{
  functions::countint::countint,
  macros::{
    gkey::{gkey, gval},
    nvalue::nvalue,
    sizenode::sizenode,
    ttisnil::ttisnil,
    ttisnumber::ttisnumber,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
};

/// cpp `numusehash`（ltable.cpp）：统计哈希部分已用节点，整数键的区间计数写回
/// `nums`。
///
/// cpp 用 `int* pnasize` 出参累加数组部分增量并以返回值为已用节点数，Rust 版
/// 折叠为 `(ause, totaluse)` 元组返回。
///
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn numusehash(t: *const LuaTable, nums: &mut [i32]) -> (i32, i32) {
  let mut totaluse: i32 = 0; // total number of elements
  let mut ause: i32 = 0; // summation of `nums'
  let mut i: i32 = unsafe { sizenode!(t) };

  while i != 0 {
    i -= 1;

    let n: *mut LuaNode = unsafe { (*t).node.add(i as usize) };
    unsafe {
      if !ttisnil!(gval!(n)) {
        if ttisnumber!(gkey!(n)) {
          let key = nvalue!(gkey!(n));
          ause += countint(key, nums);
        }
        totaluse += 1;
      }
    }
  }

  (ause, totaluse)
}
