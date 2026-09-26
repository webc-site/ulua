use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
  ast_stat_for_in::AstStatForIn, ast_stat_function::AstStatFunction,
  ast_stat_local_function::AstStatLocalFunction, ast_stat_return::AstStatReturn, ast_type::AstType,
  ast_visitor::AstVisitor,
};
#[derive(Debug, Clone, Default)]
pub struct ContainsFunctionCall {
  pub(crate) also_return: bool,
  pub(crate) result: bool,
}

impl ContainsFunctionCall {
  pub fn new(also_return: bool) -> Self {
    Self {
      also_return,
      result: false,
    }
  }
}

impl AstVisitor for ContainsFunctionCall {
  fn visit_expr(&mut self, _node: &mut AstExpr) -> bool {
    // short circuit if result is true
    !self.result
  }

  fn visit_expr_call(&mut self, _node: &mut AstExprCall) -> bool {
    self.result = true;
    false
  }

  fn visit_stat_for_in(&mut self, _node: &mut AstStatForIn) -> bool {
    // for in loops perform an implicit function call as part of the iterator protocol
    self.result = true;
    false
  }

  fn visit_stat_return(&mut self, node: &mut AstStatReturn) -> bool {
    if self.also_return {
      self.result = true;
      false
    } else {
      self.visit_stat(&mut node.base)
    }
  }

  fn visit_expr_function(&mut self, _node: &mut AstExprFunction) -> bool {
    false
  }

  fn visit_stat_function(&mut self, _node: &mut AstStatFunction) -> bool {
    false
  }

  fn visit_stat_local_function(&mut self, _node: &mut AstStatLocalFunction) -> bool {
    false
  }

  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    true
  }
}
