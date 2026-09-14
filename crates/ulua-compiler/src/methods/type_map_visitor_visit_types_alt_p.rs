use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_constant_number::AstExprConstantNumber, ast_type::AstType,
};

use crate::records::type_map_visitor::TypeMapVisitor;

impl<'a> TypeMapVisitor<'a> {
  pub fn visit_ast_expr_constant_number(&mut self, node: *mut AstExprConstantNumber) -> bool {
    if node.is_null() {
      return false;
    }

    // builtin_types.number_type is an AstTypeReference (or similar wrapper).
    // The record_resolved_type_ast_expr_ast_type method expects *const AstType.
    // In Luau AST, AstTypeReference contains a base AstType at offset 0.
    let ty_ptr = &self.builtin_types.number_type as *const _ as *const AstType;

    self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, ty_ptr);

    false
  }
}
