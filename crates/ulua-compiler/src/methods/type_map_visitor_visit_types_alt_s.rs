use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString, ast_type::AstType,
};

use crate::records::type_map_visitor::TypeMapVisitor;

impl TypeMapVisitor<'_> {
  pub fn visit_ast_expr_interp_string(&mut self, node: *mut AstExprInterpString) -> bool {
    self.record_resolved_type_ast_expr_ast_type(
      node as *mut AstExpr,
      &self.builtin_types.string_type as *const _ as *const AstType,
    );
    false
  }
}
