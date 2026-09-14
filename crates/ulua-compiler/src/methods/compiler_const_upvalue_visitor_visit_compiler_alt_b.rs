use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::records::const_upvalue_visitor::ConstUpvalueVisitor;

impl ConstUpvalueVisitor {
  pub fn visit_ast_expr_function(&mut self, _node: *mut AstExprFunction) -> bool {
    false
  }
}
