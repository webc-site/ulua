use ulua_ast::records::ast_expr_table::AstExprTable;

use crate::records::lint_multi_line_statement::LintMultiLineStatement;
impl LintMultiLineStatement {
  pub fn visit_ast_expr_table(&mut self, _node: *mut AstExprTable) -> bool {
    false
  }
}
