use core::{cmp::min, ptr};

use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_block::AstStatBlock, position::Position},
  rtti::AstNodePtr,
};

use crate::records::arena_handle::Handle;

/// 对应 C++ `blockDiffStart`（FragmentAutocomplete.cpp:298 起）：新旧 AST 根块
/// 自头部逐条比对语句，返回第一处差异的起始 Position；nearest 即新块根本身、
/// 或新块比旧块短等无从对齐情形返回 `None`。
///
/// §2：`block_old`/`block_new` 为 cpp 无条件解引用的 `AstStatBlock*`（非空前提
/// 由 fragment 差异驱动方的构造点保证），入口已收非空 [`Handle`]；
/// `nearest_statement_new_ast` 的可空槽（cpp `AstStat*`，null 无从对齐恒折回
/// `None`）以 `Option` 表达，`None` 早退与原 null 路径逐位等价。
pub fn block_diff_start(
  block_old: Handle<AstStatBlock>,
  block_new: Handle<AstStatBlock>,
  nearest_statement_new_ast: Option<Handle<AstStat>>,
) -> Option<Position> {
  // `Handle` 契约：两旧/新 AST 根由 fragment 差异驱动方传入、parser bump
  // arena 持有，bump 块地址不移动、比本函数长寿；本函数单线程只读遍历，
  // 无并存可变借用。
  let block_old = block_old.get();
  let block_new = block_new.get();

  let old_body = &block_old.body;
  let new_body = &block_new.body;
  let old_size = old_body.len();
  let new_size = new_body.len();

  // 可空槽先折叠：null nearest（cpp `nullptr`）在原实现中经「不等于根」判定后
  // 在下标搜索处必然失配、终归 `None`，此处直接早退，逐位等价。
  let nearest_statement_new_ast = nearest_statement_new_ast?;

  // We couldn't find a nearest statement
  if ptr::eq(
    nearest_statement_new_ast.as_ptr().as_ast_node(),
    &block_new.base.base,
  ) {
    return None;
  }

  // nearest 在新块中的下标（C++ 的 found 循环）
  let st_index = new_body
    .iter_nodes()
    .position(|st| st.as_ptr() == nearest_statement_new_ast.as_ptr())?;

  // Take care of some easy cases!
  if old_size == 0 && !new_body.is_empty() {
    let location = new_body[0].base.location;
    return Some(location.begin);
  }

  if new_size < old_size {
    return None;
  }

  // i < min(old_size, st_index + 1) <= old_stats.len() 且 <= new_stats.len()，
  // zip+take 单遍历，消除越界检查
  let min_len = min(old_size, st_index + 1);
  for (old_stat, new_stat) in old_body.iter().zip(new_body.iter()).take(min_len) {
    let is_same = old_stat.base.class_index == new_stat.base.class_index
      && old_stat.base.location == new_stat.base.location;
    if !is_same {
      return Some(old_stat.base.location.begin);
    }
  }

  if old_size <= st_index {
    let location = new_body[old_size].base.location;
    return Some(location.begin);
  }

  None
}
