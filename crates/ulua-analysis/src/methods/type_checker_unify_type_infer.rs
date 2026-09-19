use ulua_ast::records::location::Location;

use crate::{
  records::{type_checker::TypeChecker, unifier_options::UnifierOptions},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn unify_type_id_type_id_scope_ptr_location(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: &ScopePtr,
    location: &Location,
  ) -> bool {
    let options = UnifierOptions::default();
    self.unify_type_id_type_id_scope_ptr_location_unifier_options(
      sub_ty, super_ty, scope, location, &options,
    )
  }
}
