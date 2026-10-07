use core::ptr::{from_mut, null_mut};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_node::AstNode, ast_stat_return::AstStatReturn, ast_visitor::AstVisitor,
};
#[derive(Debug, Clone)]
pub struct Visitor {
  pub(crate) result: *mut AstStatReturn,
}

impl Visitor {
  pub fn new() -> Self {
    // 既有约定（review.md §2）：result 空 = 尚未命中首个非空 `return`，是指针身份结果字段
    // （判读收口于本文件 `visit_stat_return` 的 `is_null`），与 records/find_node.rs `best` 同族保留。
    Self { result: null_mut() }
  }
}

impl Default for Visitor {
  fn default() -> Self {
    Self::new()
  }
}

impl AstVisitor for Visitor {
  fn visit_expr(&mut self, _node: &mut AstExpr) -> bool {
    false
  }

  fn visit_stat_return(&mut self, node: &mut AstStatReturn) -> bool {
    if self.result.is_null() && !node.list.is_empty() {
      self.result = from_mut(node);
    }
    false
  }

  fn visit_node(&mut self, _node: &mut AstNode) -> bool {
    false
  }
}
