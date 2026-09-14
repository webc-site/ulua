use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_type_alias(
    &mut self,
    type_alias: *mut AstStatTypeAlias,
  ) -> NonStrictContext {
    unsafe {
      let type_alias = &*type_alias;

      self.visit_generics(type_alias.generics, type_alias.generic_packs);
      self.visit_ast_type(type_alias.type_ptr);

      NonStrictContext::new()
    }
  }
}
