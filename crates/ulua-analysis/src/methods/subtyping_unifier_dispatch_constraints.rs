//! Source: `Analysis/src/SubtypingUnifier.cpp:33-52` — `SubtypingUnifier::dispatchConstraints`.

use alloc::vec::Vec;
use core::ptr::null;

use crate::{
  enums::unify_result::UnifyResult,
  records::{constraint::Constraint, result::Result, subtyping_unifier::SubtypingUnifier},
  type_aliases::{constraint_v::ConstraintV, upper_bounds::UpperBounds},
};
impl SubtypingUnifier {
  /// # Safety
  /// 调用方须保证 `constraint` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn dispatch_constraints(
    &self,
    constraint: *const Constraint,
    assumed_constraints: Vec<ConstraintV>,
  ) -> Result {
    let mut unifier_res = UnifyResult::Ok;
    // NOTE: You *could* potentially reuse the input vector, but this seems
    // easier to read.
    let mut outstanding_constraints: Vec<ConstraintV> =
      Vec::with_capacity(assumed_constraints.len());
    let mut upper_bounds: UpperBounds = UpperBounds::new(null());
    for cv in assumed_constraints {
      let (unified, dispatched) =
        unsafe { self.dispatch_one_constraint(constraint, &cv, &mut upper_bounds) };
      unifier_res &= unified;
      if !dispatched {
        outstanding_constraints.push(cv);
      }
    }
    Result {
      unified: unifier_res,
      outstanding_constraints,
      upper_bound_contributors: upper_bounds,
    }
  }
}
