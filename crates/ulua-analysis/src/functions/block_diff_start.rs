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
  let mut st_index: usize = 0;

  // We couldn't find a nearest statement
  if eq(
    nearest_statement_new_ast as *mut AstNode,
    &block_new.base.base,
  ) {
    return None;
  }

  let mut found = false;
  let mut i = 0;
  while i < _new.size {
    let st = unsafe { *_new.data.add(i) };
    if st == nearest_statement_new_ast {
      found = true;
      st_index = i;
      break;
    }
    i += 1;
  }

  if !found {
    return None;
  }

  // Take care of some easy cases!
  if old_size == 0 && _new.size > 0 {
    let first_stat = unsafe { *_new.data };
    let location = unsafe { (*first_stat).base.location };
    return Some(location.begin);
  }

  if _new.size < old_size {
    return None;
  }

  let min_val = if old_size < st_index + 1 {
    old_size
  } else {
    st_index + 1
  };
  let mut j = 0;
  while j < min_val {
    let old_stat = unsafe { *_old.data.add(j) };
    let new_stat = unsafe { *_new.data.add(j) };

    let is_same = unsafe {
      (*old_stat).base.class_index == (*new_stat).base.class_index
        && (*old_stat).base.location == (*new_stat).base.location
    };
    if !is_same {
      let location = unsafe { (*old_stat).base.location };
      return Some(location.begin);
    }
    j += 1;
  }

  if old_size <= st_index {
    let stat_at_old_size = unsafe { *_new.data.add(old_size) };
    let location = unsafe { (*stat_at_old_size).base.location };
    return Some(location.begin);
  }

  None
}
