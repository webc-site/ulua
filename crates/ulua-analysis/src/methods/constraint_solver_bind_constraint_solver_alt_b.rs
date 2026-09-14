//! `void ConstraintSolver::bind(NotNull<const Constraint> constraint, TypePackId tp, TypePackId bound_to)`
//! (`Analysis/src/ConstraintSolver.cpp:982-1001`, hand-ported faithfully).

use alloc::string::String;
use std::ptr::eq;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::occurs_check_result::OccursCheckResult,
  functions::{
    as_mutable_type_pack::as_mutable_type_pack_id, follow_type_pack::follow_type_pack_id,
    get_type_pack::get_type_pack_id,
    occurs_check_type_utils_alt_b::occurs_check_type_pack_id_type_pack_id,
  },
  methods::unifiable_bound_type_pack_id_emplace_type_pack_bound_type_pack::emplace_type_pack,
  records::{
    blocked_type_pack::BlockedTypePack, constraint::Constraint,
    constraint_solver::ConstraintSolver, free_type_pack::FreeTypePack,
    internal_error::InternalError,
  },
  type_aliases::type_pack_id::TypePackId,
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn bind_not_null_constraint_type_pack_id_type_pack_id(
    &mut self,
    constraint: *const Constraint,
    tp: TypePackId,
    bound_to: TypePackId,
  ) {
    LUAU_ASSERT!(
      get_type_pack_id::<BlockedTypePack>(tp).is_some()
        || get_type_pack_id::<FreeTypePack>(tp).is_some()
    );
    LUAU_ASSERT!(can_mutate_type_pack_id(tp, constraint));

    let bound_to = unsafe { follow_type_pack_id(bound_to) };
    LUAU_ASSERT!(tp != bound_to);

    // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
    let location = unsafe { (*constraint).location };

    if FFlag::LuauOccursCheckForAllBindings.get()
      && occurs_check_type_pack_id_type_pack_id(tp, bound_to) == OccursCheckResult::Fail
    {
      self.report_error_type_error_data_location(
        InternalError {
          message: String::from("Attempted to create a type pack cycle"),
        }
        .into(),
        &location,
      );
      let mutable_tp = as_mutable_type_pack_id(tp);
      // SAFETY: builtin_types 在 solver 存活期内有效。
      let mut err_arg = unsafe { (*self.builtin_types).error_type_pack };
      unsafe { emplace_type_pack(mutable_tp, &mut err_arg) };
    } else {
      let mutable_tp = as_mutable_type_pack_id(tp);
      let mut bound_arg = bound_to;
      unsafe { emplace_type_pack(mutable_tp, &mut bound_arg) };
    }

    self.unblock_type_pack_id_location(tp, location);
  }
}

// C++ `[[maybe_unused]] static bool canMutate(TypePackId tp, NotNull<const Constraint> constraint)`
// (`Analysis/src/ConstraintSolver.cpp:101-111`), used only in asserts.
fn can_mutate_type_pack_id(tp: TypePackId, constraint: *const Constraint) -> bool {
  if let Some(blocked) = get_type_pack_id::<BlockedTypePack>(tp) {
    let owner = blocked.owner;
    LUAU_ASSERT!(!owner.is_null());
    return eq(owner, constraint);
  }
  true
}
