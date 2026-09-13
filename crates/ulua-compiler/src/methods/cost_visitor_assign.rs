use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_node::AstNode},
  rtti::ast_node_as,
};

use crate::records::cost_visitor::CostVisitor;

impl CostVisitor {
  pub(crate) fn assign(&mut self, expr: *mut AstExpr) {
    cost_visitor_assign(self, expr);
  }
}

pub fn cost_visitor_assign(visitor: &mut CostVisitor, expr: *mut AstExpr) {
  unsafe {
    if expr.is_null() {
      return;
    }

    let expr_local = ast_node_as::<AstExprLocal>(expr as *mut AstNode);
    if expr_local.is_null() {
      return;
    }

    let local = (*expr_local).local;
    if local.is_null() {
      return;
    }

    let key = &local;
    if let Some(found) = visitor.vars.find_mut(key) {
      *found = 0;
    }
  }
}
