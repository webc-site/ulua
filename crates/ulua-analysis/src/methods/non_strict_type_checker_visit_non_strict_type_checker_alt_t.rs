use ulua_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_declare_global(
    &mut self,
    decl_global: *mut AstStatDeclareGlobal,
  ) -> NonStrictContext {
    let type_ = unsafe { (*decl_global).type_ };
    self.visit_ast_type(type_);
    NonStrictContext::new()
  }
}
