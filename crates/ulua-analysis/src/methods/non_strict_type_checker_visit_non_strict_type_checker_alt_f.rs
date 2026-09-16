use ulua_ast::records::ast_stat_break::AstStatBreak;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  pub fn visit_ast_stat_break(&mut self, _break_statement: *mut AstStatBreak) {}
}
