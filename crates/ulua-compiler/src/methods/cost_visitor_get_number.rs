use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{constant::Constant, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn get_number(&self, node: *mut AstExpr, result: &mut f64) -> bool {
    unsafe {
      if let Some(Constant::Number(n)) = (*self.constants).find(&node) {
        *result = *n;
        return true;
      }
    }
    false
  }
}
