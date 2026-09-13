use ulua_ast::records::ast_stat_declare_function::AstStatDeclareFunction;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_declare_function(
    &mut self,
    decl_fn: *mut AstStatDeclareFunction,
  ) -> NonStrictContext {
    unsafe {
      let generics = (*decl_fn).generics;
      let generic_packs = (*decl_fn).generic_packs;
      let params = (*decl_fn).params;
      let ret_types = (*decl_fn).ret_types;

      self.visit_generics(generics, generic_packs);
      self.visit_ast_type_list(&mut params.clone());
      self.visit_ast_type_pack(ret_types);
    }

    NonStrictContext::new()
  }
}
