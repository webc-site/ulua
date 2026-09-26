//! Source: `Compiler/src/Compiler.cpp`

use ulua_ast::records::{
  ast_expr::AstExpr, ast_stat_return::AstStatReturn, ast_visitor::AstVisitor,
};

use crate::records::compiler::Compiler;

/// cpp `ReturnVisitor`：判定函数是否恒返回单值。
/// `compiler` 借用为只读（查询 `is_expr_mult_ret`），取代裸指针自引用。
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReturnVisitor<'a> {
  pub(crate) compiler: &'a Compiler,
  pub(crate) returns_one: bool,
}

impl AstVisitor for ReturnVisitor<'_> {
  fn visit_expr(&mut self, _node: &mut AstExpr) -> bool {
    false
  }

  fn visit_stat_return(&mut self, node: &mut AstStatReturn) -> bool {
    self.returns_one &=
      node.list.size == 1 && !self.compiler.is_expr_mult_ret(node.list.as_slice()[0]);
    false
  }
}

impl<'a> ReturnVisitor<'a> {
  /// 构造：初值 `returns_one = true`（cpp `ReturnVisitor` 成员初始化）。原
  /// `Compiler::return_visitor_return_visitor` 样板转发收敛到被构造类型自身。
  pub(crate) fn new(compiler: &'a Compiler) -> Self {
    Self {
      compiler,
      returns_one: true,
    }
  }
}
