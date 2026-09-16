//! @interface-stub
use core::ptr::null_mut;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{
    checkpoint::Checkpoint, constraint::Constraint, constraint_generator::ConstraintGenerator,
    pack_subtype_constraint::PackSubtypeConstraint,
  },
  type_aliases::constraint_v::ConstraintVMember,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn add_all_as_dependencies_and_chain_returns(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  target: *mut Constraint,
) {
  LUAU_ASSERT!(FFlag::LuauConstraintGraph.get());

  let mut previous: *mut Constraint = null_mut();

  for i in start.offset..end.offset {
    let constraint = cg.constraints[i];

    unsafe {
      (*cg.cgraph).add_dependency_of_constraint_constraint(&mut *constraint, &mut *target);

      if let Some(psc) = PackSubtypeConstraint::get_if(&(*constraint).c)
        && psc.returns
      {
        if !previous.is_null() {
          (*cg.cgraph).add_dependency_of_constraint_constraint(&mut *previous, &mut *constraint);
        }

        previous = constraint;
      }
    }
  }
}
