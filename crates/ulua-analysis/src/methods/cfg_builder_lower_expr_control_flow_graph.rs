use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_local::AstExprLocal,
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
  pub fn lower_expr_ast_expr(&mut self, expr: *mut AstExpr) {
    unsafe {
      let local = ast_node_as::<AstExprLocal>(expr as *mut AstNode);
      if !local.is_null() {
        self.lower_expr_ast_expr_local(local);
      } else {
        let binop = ast_node_as::<AstExprBinary>(expr as *mut AstNode);
        if !binop.is_null() {
          LUAU_ASSERT!(!(*binop).left.is_null());
          LUAU_ASSERT!(!(*binop).right.is_null());
          self.lower_expr_ast_expr((*binop).left);
          self.lower_expr_ast_expr((*binop).right);
        }
      }
    }
  }
}
