use core::{
  cmp::min,
  ptr::{self, NonNull},
};

use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_block::AstStatBlock, position::Position},
  rtti::AstNodePtr,
};
/// 对应 C++ `blockDiffStart`（FragmentAutocomplete.cpp:298 起）：新旧 AST 根块
/// 自头部逐条比对语句，返回第一处差异的起始 Position；nearest 即新块根本身、
/// 或新块比旧块短等无从对齐情形返回 `None`。
///
/// `block_old`/`block_new` 为 cpp 无条件解引用的 `AstStatBlock*`（非空前提由
/// fragment 差异驱动方的构造点保证），故用 checked `NonNull` 物化：null 输入
/// 在 Rust 侧表现为 panic 而非 cpp 的 UB。
pub fn block_diff_start(
  block_old: *mut AstStatBlock,
  block_new: *mut AstStatBlock,
  nearest_statement_new_ast: *mut AstStat,
) -> Option<Position> {
  // SAFETY: block_old 是差异比对的旧 AST 根 AstStatBlock，由 fragment 差异驱动方传入、
  // parser bump arena 持有，bump 块地址不移动、比本函数长寿；本函数单线程只读遍历，
  // 无并存可变借用。NonNull::new 兜住 null（cpp 直 deref 为 UB，此处改 panic）。
  let block_old = unsafe {
    NonNull::new(block_old)
      .expect("blockDiffStart 旧 AST 根非空（cpp 首行直 deref）")
      .as_ref()
  };
  // SAFETY: block_new 同上为新 AST 根，存活非空，仅只读访问，单线程独占。
  let block_new = unsafe {
    NonNull::new(block_new)
      .expect("blockDiffStart 新 AST 根非空（cpp 首行直 deref）")
      .as_ref()
  };

  let old_body = &block_old.body;
  let new_body = &block_new.body;
  let old_size = old_body.len();
  let new_size = new_body.len();

  // We couldn't find a nearest statement
  if ptr::eq(
    nearest_statement_new_ast.as_ast_node(),
    &block_new.base.base,
  ) {
    return None;
  }

  // nearest 在新块中的下标（C++ 的 found 循环）
  let st_index = new_body
    .iter_nodes()
    .position(|st| st.as_ptr() == nearest_statement_new_ast)?;

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
