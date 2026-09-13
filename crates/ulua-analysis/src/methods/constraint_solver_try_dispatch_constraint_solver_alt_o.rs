use alloc::vec::Vec;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::polarity::Polarity,
  functions::{
    as_mutable_type::as_mutable_type_id, extend_type_pack::extend_type_pack,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id, fresh_type::fresh_type,
    get_type_alt_j::get_type_id, track_interior_free_type::track_interior_free_type,
  },
  records::{
    blocked_type::BlockedType, constraint::Constraint, constraint_solver::ConstraintSolver,
    pending_expansion_type::PendingExpansionType, unpack_constraint::UnpackConstraint,
  },
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_unpack_constraint_not_null_constraint(
    &mut self,
    c: &UnpackConstraint,
    constraint: *const Constraint,
  ) -> bool {
    let source_pack = unsafe { follow_type_pack_id(c.source_pack) };

    if self.is_blocked_type_pack_id(source_pack) {
      return self.block_type_pack_id_not_null_constraint(source_pack, constraint);
    }

    let src_pack = unsafe {
      extend_type_pack(
        // SAFETY: arena 在 solver 存活期内有效。
        &mut *self.arena,
        self.builtin_types,
        source_pack,
        c.result_pack.len(),
        Vec::new(),
      )
    };

    let mut i = 0;
    while i < c.result_pack.len() {
      if i >= src_pack.head.len() {
        break;
      }

      let src_ty = follow_type_id(src_pack.head[i]);
      let result_ty = follow_type_id(c.result_pack[i]);

      if get_type_id::<BlockedType>(result_ty).is_some() {
        LUAU_ASSERT!(can_mutate_type_id(result_ty, constraint));

        if follow_type_id(src_ty) == result_ty {
          // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
          let scope = unsafe { (*constraint).scope };
          let fresh_ty = fresh_type(
            // SAFETY: arena 在 solver 存活期内有效。
            unsafe { &mut *self.arena },
            // SAFETY: builtin_types 在 solver 存活期内有效。
            unsafe { &*self.builtin_types },
            scope,
            Polarity::Positive,
          );
          track_interior_free_type(scope, fresh_ty);

          if FFlag::LuauConstraintGraph.get() {
            unsafe {
              self.bind_not_null_constraint_type_id_type_id(constraint, result_ty, fresh_ty)
            };
          } else {
            self.deprecate_d_shift_references(result_ty, fresh_ty);
            unsafe {
              (*as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(fresh_ty);
            }
          }
        } else {
          unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, result_ty, src_ty) };
        }
      } else {
        self.constraint_solver_unify(constraint, src_ty, result_ty);
      }

      if !FFlag::LuauConstraintGraph.get() {
        self.unblock_type_id_location(result_ty, unsafe { (*constraint).location });
      }

      i += 1;
    }

    while i < c.result_pack.len() {
      let result_ty = follow_type_id(c.result_pack[i]);
      LUAU_ASSERT!(can_mutate_type_id(result_ty, constraint));

      if get_type_id::<BlockedType>(result_ty).is_some()
        || get_type_id::<PendingExpansionType>(result_ty).is_some()
      {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            result_ty,
            (*self.builtin_types).nil_type,
          )
        };
      }

      i += 1;
    }

    true
  }
}

fn can_mutate_type_id(ty: TypeId, constraint: *const Constraint) -> bool {
  if let Some(blocked) = get_type_id::<BlockedType>(ty) {
    let owner = blocked.get_owner();
    LUAU_ASSERT!(!owner.is_null());
    return owner == constraint;
  }

  true
}
