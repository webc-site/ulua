//! Source: `Compiler/src/Compiler.cpp`

use alloc::vec::Vec;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_local::AstExprLocal,
  ast_local::AstLocal, ast_visitor::AstVisitor,
};

use crate::records::{compiler::Compiler, node::Node};

/// cpp `ConstUpvalueVisitor`：收集被闭包捕获的常量 local。
/// `compiler` 借用为只读（查询 `is_constant`），取代裸指针自引用。
#[derive(Debug)]
pub(crate) struct ConstUpvalueVisitor<'a> {
  pub(crate) compiler: &'a Compiler,
  pub(crate) upvals: Vec<Node<AstLocal>>,
}

impl AstVisitor for ConstUpvalueVisitor<'_> {
  fn visit_expr_local(&mut self, node: &mut AstExprLocal) -> bool {
    if node.upvalue
      && self
        .compiler
        .is_constant(Node::from_mut(node).cast::<AstExpr>())
    {
      self.upvals.push(node.local.into());
    }
    false
  }

  fn visit_expr_function(&mut self, _node: &mut AstExprFunction) -> bool {
    false
  }
}

impl<'a> ConstUpvalueVisitor<'a> {
  /// 构造：收集表为空。原 `Compiler::const_upvalue_visitor_const_upvalue_visitor`
  /// 样板转发收敛到被构造类型自身。
  pub(crate) fn new(compiler: &'a Compiler) -> Self {
    Self {
      compiler,
      upvals: Vec::new(),
    }
  }
}
