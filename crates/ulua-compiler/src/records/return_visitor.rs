//! Source: `Compiler/src/Compiler.cpp`

use ulua_ast::records::{
  ast_expr::AstExpr, ast_stat_return::AstStatReturn, ast_visitor::AstVisitor,
};

use crate::{functions::ast_slot_ref::ast_slot_ref, records::compiler::Compiler};

/// cpp `ReturnVisitor`：判定函数是否恒返回单值。
/// `is_expr_mult_ret` 下钻可能 intern 常量表键名，故本 visitor 独占借用
/// `&mut Compiler`（取代旧只读自引用）。
#[derive(Debug)]
pub(crate) struct ReturnVisitor<'a> {
  pub(crate) compiler: &'a mut Compiler,
  pub(crate) returns_one: bool,
}

impl AstVisitor for ReturnVisitor<'_> {
  fn visit_expr(&mut self, _node: &mut AstExpr) -> bool {
    false
  }

  fn visit_stat_return(&mut self, node: &mut AstStatReturn) -> bool {
    // size==1 守卫下首槽存在；槽位门面把 null 折叠为「非多返回值」，与 cpp 同构。
    self.returns_one &= node.list.size == 1
      && !node
        .list
        .as_slice()
        .first()
        .copied()
        .and_then(ast_slot_ref)
        .is_some_and(|expr| self.compiler.is_expr_mult_ret(expr));
    false
  }
}

impl<'a> ReturnVisitor<'a> {
  /// 构造：初值 `returns_one = true`（cpp `ReturnVisitor` 成员初始化）。原
  /// `Compiler::return_visitor_return_visitor` 样板转发收敛到被构造类型自身。
  pub(crate) fn new(compiler: &'a mut Compiler) -> Self {
    Self {
      compiler,
      returns_one: true,
    }
  }
}
