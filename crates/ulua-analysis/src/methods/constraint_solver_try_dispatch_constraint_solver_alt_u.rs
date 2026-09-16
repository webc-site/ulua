use ulua_common::{FFlag::LuauExplicitTypeInstantiationSupport, macros::luau_assert::LUAU_ASSERT};

use crate::records::{
  constraint::Constraint, constraint_solver::ConstraintSolver,
  type_instantiation_constraint::TypeInstantiationConstraint,
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_type_instantiation_constraint_not_null_constraint(
    &mut self,
    c: &TypeInstantiationConstraint,
    constraint: *const Constraint,
  ) -> bool {
    LUAU_ASSERT!(LuauExplicitTypeInstantiationSupport.get());

    if self.is_blocked_type_id(c.function_type) {
      return self.block_type_id_not_null_constraint(c.function_type, constraint);
    }

    let bound_to = self.instantiate_function_type(
      c.function_type,
      &c.type_arguments,
      &c.type_pack_arguments,
      unsafe { (*constraint).scope },
      unsafe { &(*constraint).location },
    );
    unsafe {
      self.bind_not_null_constraint_type_id_type_id(constraint, c.placeholder_type, bound_to)
    };

    true
  }
}
