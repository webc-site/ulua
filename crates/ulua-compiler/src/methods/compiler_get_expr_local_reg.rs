use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_node::AstNode},
  rtti::ast_node_as,
};
use ulua_common::FFlag::DebugLuauUserDefinedClasses;

use crate::records::compiler::Compiler;

impl Compiler {
  pub fn get_expr_local_reg(&mut self, node: *mut AstExpr) -> i32 {
    unsafe {
      let expr = self.get_expr_local(node);
      if !expr.is_null() {
        match self.locals.find(&(*expr).local) {
          Some(l) if l.allocated => l.reg as i32,
          _ => -1,
        }
      } else if DebugLuauUserDefinedClasses.get() {
        let g = ast_node_as::<AstExprGlobal>(node as *mut AstNode);
        if !g.is_null()
          && let Some(&local) = self.class_locals.find(&(*g).name)
        {
          return self.get_local_reg(local);
        }
        -1
      } else {
        -1
      }
    }
  }
}
