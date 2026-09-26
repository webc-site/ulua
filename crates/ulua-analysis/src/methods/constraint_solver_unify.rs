//! `template<typename TID>
//!  bool ConstraintSolver::unify(NotNull<const Constraint> constraint, TID sub_ty, TID super_ty)`
//! (`Analysis/src/ConstraintSolver.cpp:3833-3871`, hand-ported faithfully).

/// Models the C++ `static_assert(std::is_same_v<TID, TypeId> || std::is_same_v<TID, TypePackId>)`
/// plus the `if constexpr` split over the `Subtyping::isSubtype` overload.
use alloc::vec::Vec;
use core::{mem::take, ptr::NonNull};

use crate::{
  enums::unify_result::UnifyResult,
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    internal_error_reporter::InternalErrorReporter, occurs_check_failed::OccursCheckFailed,
    scope::Scope, subtyping::Subtyping, subtyping_result::SubtypingResult,
    subtyping_unifier::SubtypingUnifier, unification_too_complex::UnificationTooComplex,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub trait UnifyTid: Copy {
  fn is_subtype_in(
    self,
    subtyping: &mut Subtyping,
    super_ty: Self,
    scope: *mut Scope,
  ) -> SubtypingResult;
}

impl UnifyTid for TypeId {
  fn is_subtype_in(
    self,
    subtyping: &mut Subtyping,
    super_ty: Self,
    scope: *mut Scope,
  ) -> SubtypingResult {
    subtyping.is_subtype_type_id_type_id_not_null_scope(self, super_ty, scope)
  }
}

impl UnifyTid for TypePackId {
  fn is_subtype_in(
    self,
    subtyping: &mut Subtyping,
    super_ty: Self,
    scope: *mut Scope,
  ) -> SubtypingResult {
    subtyping.is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
      self,
      super_ty,
      scope,
      &Vec::new(),
    )
  }
}

impl ConstraintSolver {
  pub(crate) fn constraint_solver_unify<TID: UnifyTid>(
    &mut self,
    constraint: *const Constraint,
    sub_ty: TID,
    super_ty: TID,
  ) -> bool {
    // Safety: `constraint` 对应 C++ `NotNull<const Constraint>` 形参——调用方一律以
    // `NonNull<Constraint>::as_ptr()`（solver 自身 push_constraint 的产物）传入，故非空、
    // 对齐且指向 constraint arena 里的存活结点。这里只在该结点存活期内一次性拷贝
    // `scope`（裸指针句柄，本身不解引用）与 `location`（`Copy`），刻意不留 `&Constraint`
    // 引用：随后 `push_constraint` 会向同一 arena 追加结点，`stu.dispatch_constraints`
    // 也会读取该 constraint，避免长期借用与这些写入重叠。
    let (scope, location) = unsafe { ((*constraint).scope, (*constraint).location) };
    let ice_ptr = &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter;

    let mut subtyping = Subtyping::subtyping_owned(
      self.builtin_types,
      self.arena,
      self.normalizer.as_ptr(),
      self.type_function_runtime.as_ptr(),
      ice_ptr,
    );
    let stu = SubtypingUnifier::new(self.arena, self.builtin_types, ice_ptr);

    let mut result = sub_ty.is_subtype_in(&mut subtyping, super_ty, scope);

    let unifier_result =
      stu.dispatch_constraints(constraint, take(&mut result.assumed_constraints));

    for cv in unifier_result.outstanding_constraints.iter() {
      let new_constraint = self.push_constraint(
        // Safety: scope 源自存活 constraint 登记的 NotNull<Scope> 字段，恒非空。
        NonNull::new(scope).expect("scope 为 cpp NotNull<Scope> 登记值，恒非空"),
        location,
        cv.clone(),
      );
      self.inherit_blocks(constraint, new_constraint.as_ptr());
    }

    for (ty, new_upper_bounds) in unifier_result.upper_bound_contributors.iter() {
      let upper_bounds = self.upper_bound_contributors.get_or_insert(*ty);
      upper_bounds.extend(new_upper_bounds.iter().cloned());
    }

    match unifier_result.unified {
      UnifyResult::OccursCheckFailed => {
        self.report_error_type_error_data_location(OccursCheckFailed::default().into(), &location);
        false
      }
      UnifyResult::TooComplex => {
        self.report_error_type_error_data_location(
          UnificationTooComplex::default().into(),
          &location,
        );
        false
      }
      UnifyResult::Ok => true,
    }
  }
}
