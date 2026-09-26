//! 内联/展开编译后回滚常量折叠改动（cpp `ConstantFolding.cpp` undoChanges*）。
//!
//! Expr/Local 两类 change log 结构同形（键 + 旧值 + 缺席标记），回滚主体
//! 以泛型键收口一份，两个公开入口退化为类型具名薄包装。

use core::hash::Hash;

use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    constant::Constant, expr_constant_change::ExprConstantChange,
    local_constant_change::LocalConstantChange, node::Node,
  },
  type_aliases::{
    expr_constant_change_log::ExprConstantChangeLog,
    local_constant_change_log::LocalConstantChangeLog,
  },
};

/// 回滚条目的统一视图（键类型 K 具名化后即可共用回滚主体，也让折叠期的
/// 日志登记共用同一份实现）。
pub(crate) trait ChangeEntry<K> {
  fn key(&self) -> K;
  fn old_value(&self) -> Constant;
  fn was_absent(&self) -> bool;
  /// 条目唯一构造点：折叠期登记日志用，字段与回滚侧读取的一一对应。
  fn from_parts(key: K, old_value: Constant, was_absent: bool) -> Self;
}

impl ChangeEntry<Node<AstExpr>> for ExprConstantChange {
  fn key(&self) -> Node<AstExpr> {
    self.key
  }
  fn old_value(&self) -> Constant {
    self.old_value
  }
  fn was_absent(&self) -> bool {
    self.was_absent
  }
  fn from_parts(key: Node<AstExpr>, old_value: Constant, was_absent: bool) -> Self {
    Self {
      key,
      old_value,
      was_absent,
    }
  }
}

impl ChangeEntry<Node<AstLocal>> for LocalConstantChange {
  fn key(&self) -> Node<AstLocal> {
    self.key
  }
  fn old_value(&self) -> Constant {
    self.old_value
  }
  fn was_absent(&self) -> bool {
    self.was_absent
  }
  fn from_parts(key: Node<AstLocal>, old_value: Constant, was_absent: bool) -> Self {
    Self {
      key,
      old_value,
      was_absent,
    }
  }
}

/// 逆序回滚：登记时缺席的键写回 Unknown 占位（cpp 语义：条目仍在则置 Unknown），
/// 否则用日志中的旧值覆盖还原。
fn undo_changes<K, Ent>(map: &mut DenseHashMap<K, Constant>, changes: &[Ent])
where
  K: Copy + Clone + PartialEq + Hash,
  Ent: ChangeEntry<K>,
{
  for it in changes.iter().rev() {
    if it.was_absent() {
      if let Some(old) = map.find_mut(&it.key()) {
        *old = Constant::Unknown;
      }
    } else {
      let old = it.old_value();
      *map.get_or_insert(it.key()) = old;
    }
  }
}

pub fn undo_changes_expr(
  constants: &mut DenseHashMap<Node<AstExpr>, Constant>,
  changes: &ExprConstantChangeLog,
) {
  undo_changes(constants, changes);
}

pub(crate) fn undo_changes_local(
  locals: &mut DenseHashMap<Node<AstLocal>, Constant>,
  changes: &LocalConstantChangeLog,
) {
  undo_changes(locals, changes);
}
