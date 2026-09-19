use core::cmp::min;
use std::ptr::eq;

use ulua_ast::records::{
  ast_node::AstNode, ast_stat::AstStat, ast_stat_block::AstStatBlock, position::Position,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn block_diff_start(
  block_old: *mut AstStatBlock,
  block_new: *mut AstStatBlock,
  nearest_statement_new_ast: *mut AstStat,
) -> Option<Position> {
  let block_old = unsafe { &*block_old };
  let block_new = unsafe { &*block_new };

  let _old = block_old.body;
  let _new = block_new.body;
  let old_size = _old.size;
  // C++ `for (auto st : _new)` —— 以切片形式遍历 AstStat* 数组
  // （`as_slice` 正确处理空数组 `data == null` 的情形）。
  let new_stats = _new.as_slice();
  let old_stats = _old.as_slice();

  // We couldn't find a nearest statement
  if eq(
    nearest_statement_new_ast as *mut AstNode,
    &block_new.base.base,
  ) {
    return None;
  }

  // nearest 在新块中的下标（C++ 的 found 循环）
  let st_index = new_stats
    .iter()
    .position(|&st| st == nearest_statement_new_ast)?;

  // Take care of some easy cases!
  if old_size == 0 && !new_stats.is_empty() {
    let first_stat = new_stats[0];
    let location = unsafe { (*first_stat).base.location };
    return Some(location.begin);
  }

  if _new.size < old_size {
    return None;
  }

  // i < min(old_size, st_index + 1) <= old_stats.len() 且 <= new_stats.len()，
  // zip+take 单遍历，消除越界检查
  let min_len = min(old_size, st_index + 1);
  for (&old_stat, &new_stat) in old_stats.iter().zip(new_stats).take(min_len) {
    let is_same = unsafe {
      (*old_stat).base.class_index == (*new_stat).base.class_index
        && (*old_stat).base.location == (*new_stat).base.location
    };
    if !is_same {
      let location = unsafe { (*old_stat).base.location };
      return Some(location.begin);
    }
  }

  if old_size <= st_index {
    let stat_at_old_size = new_stats[old_size];
    let location = unsafe { (*stat_at_old_size).base.location };
    return Some(location.begin);
  }

  None
}
