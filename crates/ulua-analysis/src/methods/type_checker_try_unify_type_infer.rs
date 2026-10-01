use ulua_ast::records::location::Location;

use crate::{
  records::type_checker::TypeChecker,
  type_aliases::{
    error_vec::ErrorVec, scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl TypeChecker {
  pub fn try_unify(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: &ScopePtr,
    location: &Location,
  ) -> ErrorVec {
    // `mk_unifier` resets the shared iteration counter (the per-top-level-unify
    // reset C++ does in the public `Unifier::tryUnify`), so this calls the
    // recursive `tryUnify_` directly, as C++ `tryUnify` does after the reset.
    let mut state = self.mk_unifier(scope, location);

    state
      .try_unify_type_id_type_id_bool_bool_literal_properties(sub_ty, super_ty, false, false, None);

    if state.errors.is_empty() {
      state.log.commit();
    }

    state.errors
  }

  /// C++ `ErrorVec TypeChecker::tryUnify(TypePackId subTp, TypePackId superTp,
  /// const ScopePtr& scope, const Location& location)` (TypeInfer.cpp).
  ///
  /// This unifies two type *packs*. The previous implementation reinterpret-cast
  /// the `TypePackId`s (`*const TypePackVar`) to `TypeId` (`*const Type`) and ran
  /// *type* unification on them — a raw-pointer type confusion: `follow` then
  /// read a `TypePackVar` as a `Type`, whose `TypeVariant` discriminant is
  /// garbage. That is layout-dependent UB (issue #6): some toolchains tolerate
  /// the bogus enum tag, others SIGSEGV. Mirror the `TypeId` overload exactly,
  /// but drive the Unifier's *pack* unify so the pointee types stay honest.
  pub fn try_unify_type_pack_id_type_pack_id_scope_ptr_location(
    &mut self,
    sub_ty: TypePackId,
    super_ty: TypePackId,
    scope: ScopePtr,
    location: &Location,
  ) -> ErrorVec {
    // `mk_unifier` resets the shared iteration counter (as the public
    // `Unifier::tryUnify` does), so call the recursive `tryUnify_` directly.
    let mut state = self.mk_unifier(&scope, location);

    state.try_unify_type_pack_id_type_pack_id_bool(sub_ty, super_ty, false);

    if state.errors.is_empty() {
      state.log.commit();
    }

    state.errors
  }
}
