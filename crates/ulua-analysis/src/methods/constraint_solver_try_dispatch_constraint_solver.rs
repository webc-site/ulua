use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::get_constraint::get_constraint,
  records::{
    assign_index_constraint::AssignIndexConstraint, assign_prop_constraint::AssignPropConstraint,
    constraint::Constraint, constraint_solver::ConstraintSolver,
    equality_constraint::EqualityConstraint, function_call_constraint::FunctionCallConstraint,
    function_check_constraint::FunctionCheckConstraint,
    generalization_constraint::GeneralizationConstraint,
    has_indexer_constraint::HasIndexerConstraint, has_prop_constraint::HasPropConstraint,
    iterable_constraint::IterableConstraint, name_constraint::NameConstraint,
    pack_subtype_constraint::PackSubtypeConstraint,
    primitive_type_constraint::PrimitiveTypeConstraint,
    push_function_type_constraint::PushFunctionTypeConstraint,
    push_type_constraint::PushTypeConstraint, reduce_constraint::ReduceConstraint,
    reduce_pack_constraint::ReducePackConstraint, simplify_constraint::SimplifyConstraint,
    subtype_constraint::SubtypeConstraint,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_instantiation_constraint::TypeInstantiationConstraint,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::constraint_vertex::ConstraintVertex,
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_not_null_constraint_bool(
    &mut self,
    constraint: *const Constraint,
    force: bool,
  ) -> bool {
    if FFlag::LuauConstraintGraph.get() {
      LUAU_ASSERT!(
        force
          || !unsafe { (*self.cgraph).has_unsolved_dependencies(ConstraintVertex::V2(constraint)) }
      );
    } else {
      if !force && self.deprecate_d_is_blocked(constraint) {
        return false;
      }
    }

    let mut success = false;
    let c = unsafe { &*constraint };

    if let Some(sc) = unsafe { get_constraint::<SubtypeConstraint>(c).as_ref() } {
      success = self.try_dispatch_subtype_constraint_not_null_constraint(sc, constraint);
    } else if let Some(psc) = unsafe { get_constraint::<PackSubtypeConstraint>(c).as_ref() } {
      success = self.try_dispatch_pack_subtype_constraint_not_null_constraint(psc, constraint);
    } else if let Some(gc) = unsafe { get_constraint::<GeneralizationConstraint>(c).as_ref() } {
      success =
        unsafe { self.try_dispatch_generalization_constraint_not_null_constraint(gc, constraint) };
    } else if let Some(ic) = unsafe { get_constraint::<IterableConstraint>(c).as_ref() } {
      success = unsafe {
        self.try_dispatch_iterable_constraint_not_null_constraint_bool(ic, constraint, force)
      };
    } else if let Some(nc) = unsafe { get_constraint::<NameConstraint>(c).as_ref() } {
      success = unsafe { self.try_dispatch_name_constraint_not_null_constraint(nc, constraint) };
    } else if let Some(taec) = unsafe { get_constraint::<TypeAliasExpansionConstraint>(c).as_ref() }
    {
      success = unsafe {
        self.try_dispatch_type_alias_expansion_constraint_not_null_constraint(taec, constraint)
      };
    } else if let Some(fcc) = unsafe { get_constraint::<FunctionCallConstraint>(c).as_ref() } {
      success = unsafe {
        self.try_dispatch_function_call_constraint_not_null_constraint_bool(fcc, constraint, force)
      };
    } else if let Some(fcc) = unsafe { get_constraint::<FunctionCheckConstraint>(c).as_ref() } {
      success = unsafe {
        self.try_dispatch_function_check_constraint_not_null_constraint_bool(fcc, constraint, force)
      };
    } else if let Some(pc) = unsafe { get_constraint::<PrimitiveTypeConstraint>(c).as_ref() } {
      success = self.try_dispatch_primitive_type_constraint_not_null_constraint(pc, constraint);
    } else if let Some(hpc) = unsafe { get_constraint::<HasPropConstraint>(c).as_ref() } {
      success = self.try_dispatch_has_prop_constraint_not_null_constraint(hpc, constraint);
    } else if let Some(spc) = unsafe { get_constraint::<HasIndexerConstraint>(c).as_ref() } {
      success = self.try_dispatch_has_indexer_constraint_not_null_constraint(spc, constraint);
    } else if let Some(uc) = unsafe { get_constraint::<AssignPropConstraint>(c).as_ref() } {
      success =
        unsafe { self.try_dispatch_assign_prop_constraint_not_null_constraint(uc, constraint) };
    } else if let Some(uc) = unsafe { get_constraint::<AssignIndexConstraint>(c).as_ref() } {
      success =
        unsafe { self.try_dispatch_assign_index_constraint_not_null_constraint(uc, constraint) };
    } else if let Some(uc) = unsafe { get_constraint::<UnpackConstraint>(c).as_ref() } {
      success = unsafe { self.try_dispatch_unpack_constraint_not_null_constraint(uc, constraint) };
    } else if let Some(rc) = unsafe { get_constraint::<ReduceConstraint>(c).as_ref() } {
      success = unsafe {
        self.try_dispatch_reduce_constraint_not_null_constraint_bool(rc, constraint, force)
      };
    } else if let Some(rpc) = unsafe { get_constraint::<ReducePackConstraint>(c).as_ref() } {
      success = unsafe {
        self.try_dispatch_reduce_pack_constraint_not_null_constraint_bool(rpc, constraint, force)
      };
    } else if let Some(eqc) = unsafe { get_constraint::<EqualityConstraint>(c).as_ref() } {
      success = self.try_dispatch_equality_constraint_not_null_constraint(eqc, constraint);
    } else if let Some(sc) = unsafe { get_constraint::<SimplifyConstraint>(c).as_ref() } {
      success = unsafe {
        self.try_dispatch_simplify_constraint_not_null_constraint_bool(sc, constraint, force)
      };
    } else if let Some(pftc) = unsafe { get_constraint::<PushFunctionTypeConstraint>(c).as_ref() } {
      success =
        self.try_dispatch_push_function_type_constraint_not_null_constraint(pftc, constraint);
    } else if let Some(esgc) = unsafe { get_constraint::<TypeInstantiationConstraint>(c).as_ref() }
    {
      LUAU_ASSERT!(FFlag::LuauExplicitTypeInstantiationSupport.get());
      success = unsafe {
        self.try_dispatch_type_instantiation_constraint_not_null_constraint(esgc, constraint)
      };
    } else if let Some(ptc) = unsafe { get_constraint::<PushTypeConstraint>(c).as_ref() } {
      success = unsafe {
        self.try_dispatch_push_type_constraint_not_null_constraint_bool(ptc, constraint, force)
      };
    } else {
      LUAU_ASSERT!(false);
    }

    success
  }
}
