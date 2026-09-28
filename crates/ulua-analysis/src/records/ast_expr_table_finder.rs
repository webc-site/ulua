use core::ptr::from_mut;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_table::AstExprTable, ast_visitor::AstVisitor};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::type_aliases::type_id::TypeId;

/// 表字面量类型收集器。引用化说明：原 `result/ast_types` 为借用裸指针，
/// 现直接以 `&mut/&` 表达同一契约（cpp `NotNull` 形参），解引用随之外移消失。
#[derive(Debug)]
pub struct AstExprTableFinder<'a> {
  pub result: &'a mut DenseHashSet<TypeId>,
  pub ast_types: &'a DenseHashMap<*const AstExpr, TypeId>,
}

impl<'a> AstExprTableFinder<'a> {
  pub fn new(
    result: &'a mut DenseHashSet<TypeId>,
    ast_types: &'a DenseHashMap<*const AstExpr, TypeId>,
  ) -> Self {
    Self { result, ast_types }
  }
}

impl AstVisitor for AstExprTableFinder<'_> {
  fn visit_expr(&mut self, _node: &mut AstExpr) -> bool {
    false
  }

  fn visit_expr_table(&mut self, node: &mut AstExprTable) -> bool {
    if let Some(ty) = self
      .ast_types
      .find(&(from_mut(&mut node.base) as *const AstExpr))
    {
      self.result.insert(*ty);
    }
    true
  }
}
