use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  pub fn visit_ast_stat_type_function(&mut self, _type_func: *mut AstStatTypeFunction) {
    // NonStrictContext visit(AstStatTypeFunction* typeFunc) { return {}; }
    // This overload is a no-op in the non-strict type checker.
  }
}
