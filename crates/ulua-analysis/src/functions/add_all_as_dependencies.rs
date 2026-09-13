//! @interface-stub
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::records::{
  checkpoint::Checkpoint, constraint::Constraint, constraint_generator::ConstraintGenerator,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn add_all_as_dependencies(
  start: Checkpoint,
  end: Checkpoint,
  cg: &ConstraintGenerator,
  target: *mut Constraint,
) {
  LUAU_ASSERT!(FFlag::LuauConstraintGraph.get());

  for i in start.offset..end.offset {
    let dependency = cg.constraints[i];

    unsafe {
      (*cg.cgraph).add_dependency_of_constraint_constraint(&mut *dependency, &mut *target);
    }
  }
}
