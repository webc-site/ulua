use ulua_ast::records::ast_stat_continue::AstStatContinue;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub fn visit_ast_stat_continue(&mut self, _continue_statement: *mut AstStatContinue) {
    let _ = NonStrictContext::new();
  }
}
