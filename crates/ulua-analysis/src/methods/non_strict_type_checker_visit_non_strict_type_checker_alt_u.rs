use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_declare_extern_type(
    &mut self,
    decl_class: *mut AstStatDeclareExternType,
  ) -> NonStrictContext {
    unsafe {
      let decl_class_ref = &*decl_class;

      if !decl_class_ref.indexer.is_null() {
        let indexer = &*decl_class_ref.indexer;
        self.visit_ast_type(indexer.index_type);
        self.visit_ast_type(indexer.result_type);
      }

      for prop in decl_class_ref.props.as_slice() {
        self.visit_ast_type(prop.ty);
      }

      NonStrictContext::new()
    }
  }
}
