use ulua_ast::records::ast_stat_break::AstStatBreak;

use crate::records::type_checker_2::TypeChecker2;

impl TypeChecker2 {
  pub fn visit_ast_stat_break(&mut self, _stat: *mut AstStatBreak) {}
}
