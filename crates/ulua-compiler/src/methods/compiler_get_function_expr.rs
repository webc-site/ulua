use core::ptr::null_mut;

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_expr_group::AstExprGroup,
    ast_expr_index_name::AstExprIndexName, ast_expr_instantiate::AstExprInstantiate,
    ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::FFlag;

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn get_function_expr(&mut self, node: *mut AstExpr) -> *mut AstExprFunction {
    unsafe {
      let expr_local = ast_node_as::<AstExprLocal>(node as *mut AstNode);
      if !expr_local.is_null() {
        let lv = self.variables.find(&(*expr_local).local);
        if lv.is_none_or(|lv| lv.written || lv.init.is_null()) {
          return null_mut();
        }

        return self.get_function_expr(lv.unwrap().init);
      }

      let expr_index = ast_node_as::<AstExprIndexName>(node as *mut AstNode);
      if !expr_index.is_null() && FFlag::LuauCompileInlineTableFunctions.get() {
        let value = self.try_index_constant_table(expr_index);
        if !value.is_null() {
          return self.get_function_expr(value);
        }

        return null_mut();
      }

      let expr_group = ast_node_as::<AstExprGroup>(node as *mut AstNode);
      if !expr_group.is_null() {
        return self.get_function_expr((*expr_group).expr);
      }

      let expr_assertion = ast_node_as::<AstExprTypeAssertion>(node as *mut AstNode);
      if !expr_assertion.is_null() {
        return self.get_function_expr((*expr_assertion).expr);
      }

      let expr_instantiate = ast_node_as::<AstExprInstantiate>(node as *mut AstNode);
      if !expr_instantiate.is_null() && FFlag::LuauCompileInlineTableFunctions.get() {
        return self.get_function_expr((*expr_instantiate).expr);
      }

      ast_node_as::<AstExprFunction>(node as *mut AstNode)
    }
  }
}
