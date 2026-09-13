use ulua_ast::records::ast_expr::AstExpr;

use crate::records::cost_visitor::CostVisitor;

impl CostVisitor {
  pub fn visit_ast_expr(&mut self, node: *mut AstExpr) -> bool {
    if node.is_null() {
      return false;
    }

    let cost = self.model(node);
    // C++ `result += model(node)` — saturating add that zeroes the constant mask.
    self.result.add_assign(&cost);

    false
  }
}
