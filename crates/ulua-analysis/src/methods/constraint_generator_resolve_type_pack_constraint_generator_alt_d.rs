use ulua_ast::records::ast_type_list::AstTypeList;

use crate::{
  enums::polarity::Polarity,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::type_pack_id::TypePackId,
};

impl ConstraintGenerator {
  pub fn resolve_type_pack_scope_ptr_ast_type_list_bool_bool_polarity(
    &mut self,
    scope: *mut Scope,
    list: &AstTypeList,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypePackId {
    let _polarity = initial_polarity;
    self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
      scope,
      list,
      in_type_arguments,
      replace_error_with_fresh,
    )
  }
}
