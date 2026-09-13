use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_group::AstExprGroup, ast_expr_local::AstExprLocal,
    ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::FFlag;

use crate::{functions::unwrap_expr_of_type::unwrap_expr_of_type, records::compiler::Compiler};

impl Compiler {
  pub fn get_expr_local(&mut self, node: *mut AstExpr) -> *mut AstExprLocal {
    unsafe {
      if FFlag::LuauCompileInlineTableFunctions.get() {
        return unwrap_expr_of_type::<AstExprLocal>(node);
      }

      let expr = ast_node_as::<AstExprLocal>(node as *mut AstNode);
      if !expr.is_null() {
        return expr;
      }

      let group = ast_node_as::<AstExprGroup>(node as *mut AstNode);
      if !group.is_null() {
        return self.get_expr_local((*group).expr);
      }

      let assertion = ast_node_as::<AstExprTypeAssertion>(node as *mut AstNode);
      if !assertion.is_null() {
        return self.get_expr_local((*assertion).expr);
      }

      null_mut()
    }
  }
}
