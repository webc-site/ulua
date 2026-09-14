use ulua_common::FFlag;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type,
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    find_all_union_members::FindAllUnionMembers, generic_type_visitor::GenericTypeVisitorTrait,
    simplify_constraint::SimplifyConstraint, union_type::UnionType,
  },
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_simplify_constraint_not_null_constraint_bool(
    &mut self,
    c: &SimplifyConstraint,
    constraint: *const Constraint,
    force: bool,
  ) -> bool {
    let target = follow_type_id(c.ty);

    unsafe {
      if (*target).persistent
        || (*target).owning_arena != self.arena
        || get_type_id::<UnionType>(target).is_none()
      {
        return true;
      }
    }

    let mut finder = FindAllUnionMembers::new();
    finder.traverse_type_id(target);

    if !finder.blocked_tys.empty() && !force {
      for ty in &finder.blocked_tys.order {
        self.block_type_id_not_null_constraint(*ty, constraint);
      }
      return false;
    }

    let mut result = unsafe { (*self.builtin_types).never_type };
    for ty in &finder.recorded_tys.order {
      let ty_followed = follow_type_id(*ty);
      if ty_followed == target {
        continue;
      }
      result = self.simplify_union(
        unsafe { (*constraint).scope },
        unsafe { (*constraint).location },
        result,
        ty_followed,
      );
    }

    if force {
      for ty in &finder.blocked_tys.order {
        let ty_followed = follow_type_id(*ty);
        if ty_followed == target {
          continue;
        }
        result = self.simplify_union(
          unsafe { (*constraint).scope },
          unsafe { (*constraint).location },
          result,
          ty_followed,
        );
      }
    }

    let mutable_target = { as_mutable_type_id(target) };
    let mut result_arg = result;
    unifiable_bound_type_id_emplace_type_bound_type(
      unsafe { &mut *mutable_target },
      &mut result_arg,
    );

    if FFlag::LuauConstraintGraph.get() {
      unsafe { (*self.cgraph).shift_references_type_id(target, result) };
    }

    true
  }
}
