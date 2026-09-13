//! `void ConstraintSolver::bind(NotNull<const Constraint> constraint, TypeId ty, TypeId bound_to)`
//! (`Analysis/src/ConstraintSolver.cpp:938-980`, hand-ported faithfully).

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type::follow_type_id, fresh_type::fresh_type,
    get_type_alt_j::get_type_id, track_interior_free_type::track_interior_free_type,
  },
  methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
  records::{
    blocked_type::BlockedType, constraint::Constraint, constraint_solver::ConstraintSolver,
    free_type::FreeType, pending_expansion_type::PendingExpansionType,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn bind_not_null_constraint_type_id_type_id(
    &mut self,
    constraint: *const Constraint,
    ty: TypeId,
    bound_to: TypeId,
  ) {
    LUAU_ASSERT!(
      get_type_id::<BlockedType>(ty).is_some()
        || get_type_id::<FreeType>(ty).is_some()
        || get_type_id::<PendingExpansionType>(ty).is_some()
    );
    LUAU_ASSERT!(can_mutate_type_id(ty, constraint));

    let bound_to = follow_type_id(bound_to);
    // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
    let scope = unsafe { (*constraint).scope };
    // SAFETY: 同上。
    let location = unsafe { (*constraint).location };

    if FFlag::LuauOccursCheckForAllBindings.get() {
      // This follow shouldn't be needed, but if for some reason we end up
      // with a bound type, we want to also follow it when doing this
      // occurence check.
      if follow_type_id(ty) == bound_to {
        let fresh_ty = fresh_type(
          // SAFETY: arena 在 solver 存活期内有效。
          unsafe { &mut *self.arena },
          // SAFETY: builtin_types 在 solver 存活期内有效。
          unsafe { &*self.builtin_types },
          scope,
          Polarity::Mixed,
        );
        let mutable_ty = as_mutable_type_id(ty);
        let mut fresh_arg = fresh_ty;
        unifiable_bound_type_id_emplace_type_bound_type(
          // SAFETY: as_mutable_type_id 返回 arena 内有效对象。
          unsafe { &mut *mutable_ty },
          &mut fresh_arg,
        );
        track_interior_free_type(scope, fresh_ty);
        self.unblock_type_id_location(ty, location);
        return;
      }
    } else if get_type_id::<BlockedType>(ty).is_some() && ty == bound_to {
      // DEPRECATED_emplace<FreeType>(constraint, ty, scope, never_type, unknown_type, Polarity::Mixed)
      // FIXME?  Is this the right polarity?
      let free_ty = FreeType::free_type_scope_type_id_type_id_polarity(
        scope,
        // SAFETY: builtin_types 在 solver 存活期内有效。
        unsafe { (*self.builtin_types).never_type },
        // SAFETY: 同上。
        unsafe { (*self.builtin_types).unknown_type },
        Polarity::Mixed,
      );
      let mutable_ty = as_mutable_type_id(ty);
      // SAFETY: as_mutable_type_id 返回 arena 内有效对象。
      unsafe {
        (*mutable_ty).ty = TypeVariant::Free(free_ty);
      }
      self.unblock_type_id_location(ty, location);
      track_interior_free_type(scope, ty);
      return;
    }

    let mutable_ty = as_mutable_type_id(ty);
    let mut bound_arg = bound_to;
    unifiable_bound_type_id_emplace_type_bound_type(
      // SAFETY: as_mutable_type_id 返回 arena 内有效对象。
      unsafe { &mut *mutable_ty },
      &mut bound_arg,
    );

    if !FFlag::LuauConstraintGraph.get() {
      // `unblock` will "shift references" under the hood.
      self.deprecate_d_shift_references(ty, bound_to);
    }

    self.unblock_type_id_location(ty, location);
  }
}

// C++ `[[maybe_unused]] static bool canMutate(TypeId ty, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:88-98`), used only in asserts.
fn can_mutate_type_id(ty: TypeId, constraint: *const Constraint) -> bool {
  if let Some(blocked) = get_type_id::<BlockedType>(ty) {
    let owner = blocked.get_owner();
    LUAU_ASSERT!(!owner.is_null());
    return owner == constraint;
  }
  true
}
