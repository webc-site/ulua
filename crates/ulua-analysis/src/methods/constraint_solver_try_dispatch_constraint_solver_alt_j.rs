use ulua_common::FFlag;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, maybe_singleton::maybe_singleton,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver, free_type::FreeType,
    pending_expansion_type::PendingExpansionType,
    primitive_type_constraint::PrimitiveTypeConstraint,
  },
  type_aliases::constraint_vertex::ConstraintVertex,
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证 `constraint` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn try_dispatch_primitive_type_constraint_not_null_constraint(
    &mut self,
    c: &PrimitiveTypeConstraint,
    constraint: *const Constraint,
  ) -> bool {
    let expected_type = c.expected_type.map(follow_type_id);

    if let Some(et) = expected_type
      && (self.is_blocked_type_id(et) || get_type_id::<PendingExpansionType>(et).is_some())
    {
      return self.block_type_id_not_null_constraint(et, constraint);
    }

    let Some(free_type) = get_type_id::<FreeType>(follow_type_id(c.free_type)) else {
      return true;
    };

    if FFlag::LuauConstraintGraph.get() {
      if unsafe {
        (*self.cgraph).has_strictly_more_than_one_dependency(ConstraintVertex::V0(c.free_type))
      } {
        self.block_type_id_not_null_constraint(c.free_type, constraint);
        return false;
      }
    } else {
      if let Some(it) = self.deprecated_type_to_constraint_set.get(&c.free_type)
        && it.len() > 1
      {
        self.block_type_id_not_null_constraint(c.free_type, constraint);
        return false;
      }
    }

    let mut bind_to = c.primitive_type;

    if free_type.upper_bound != c.primitive_type && maybe_singleton(free_type.upper_bound) {
      bind_to = free_type.lower_bound;
    } else if let Some(et) = expected_type
      && maybe_singleton(et)
    {
      bind_to = free_type.lower_bound;
    }

    let ty = follow_type_id(c.free_type);
    if !FFlag::LuauConstraintGraph.get() {
      self.deprecate_d_shift_references(ty, bind_to);
    }
    unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, ty, bind_to) };

    true
  }
}
