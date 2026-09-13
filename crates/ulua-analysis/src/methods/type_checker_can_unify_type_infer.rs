use ulua_ast::records::location::Location;

use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{error_vec::ErrorVec, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn can_unify_type_infer(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: &ScopePtr,
    location: &Location,
  ) -> ErrorVec {
    let mut state = self.mk_unifier(scope, location);
    state.can_unify_type_id_type_id(sub_ty, super_ty)
  }
}
