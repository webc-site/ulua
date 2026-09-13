use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr_table::AstExprTable, ast_type::AstType, ast_type_pack::AstTypePack,
  ast_type_table::AstTypeTable, ast_visitor::AstVisitor,
};

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintTableLiteral {
  pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintTableLiteral {
  fn visit_expr_table(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_table(node as *mut AstExprTable)
  }

  fn visit_type(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type(node as *mut AstType)
  }

  fn visit_type_pack(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_pack(node as *mut AstTypePack)
  }

  fn visit_type_table(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_type_table(node as *mut AstTypeTable)
  }
}
