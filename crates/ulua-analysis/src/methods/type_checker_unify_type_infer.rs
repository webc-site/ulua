use ulua_ast::records::location::Location;

use crate::{
  records::{
    count_mismatch::CountMismatchContext, type_checker::TypeChecker,
    unifier_options::UnifierOptions,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId},
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

  pub fn unify_type_id_type_id_scope_ptr_location_unifier_options(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: &ScopePtr,
    location: &Location,
    options: &UnifierOptions,
  ) -> bool {
    // `mk_unifier` resets the shared iteration counter (the per-top-level-unify
    // reset C++ does in the public `Unifier::tryUnify`), so this calls the
    // recursive `tryUnify_` directly, as C++ `tryUnify` does after the reset.
    let mut state = self.mk_unifier(scope, location);
    state.try_unify_type_id_type_id_bool_bool_literal_properties(
      sub_ty,
      super_ty,
      options.is_function_call,
      false,
      None,
    );

    state.log.commit();

    self.report_errors(&state.errors);

    state.errors.is_empty()
  }

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
