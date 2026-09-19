use ulua_ast::records::location::Location;

use crate::{
  records::{count_mismatch::CountMismatchContext, type_checker::TypeChecker},
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};

impl TypeChecker {
  pub fn unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
    &mut self,
    sub_ty: TypePackId,
    super_ty: TypePackId,
    scope: &ScopePtr,
    location: &Location,
    ctx: CountMismatchContext,
  ) -> bool {
    let mut state = self.mk_unifier(scope, location);
    state.ctx = ctx;
    state.try_unify_type_pack_id_type_pack_id_bool(sub_ty, super_ty, false);

    state.log.commit();

    self.report_errors(&state.errors);

    state.errors.is_empty()
  }
}
