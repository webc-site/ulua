use ulua_ast::records::ast_type::AstType;

use crate::{
  enums::polarity::Polarity,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::type_id::TypeId,
};

impl ConstraintGenerator {
  pub fn resolve_type(
    &mut self,
    _scope: *mut Scope,
    ty: *mut AstType,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypeId {
    // Reset the polarity
    self.polarity = initial_polarity;
    self.resolve_type_constraint_generator_alt_b(
      _scope,
      ty,
      in_type_arguments,
      replace_error_with_fresh,
    )
  }
}
