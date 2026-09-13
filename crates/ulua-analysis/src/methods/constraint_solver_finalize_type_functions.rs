//! `void ConstraintSolver::finalizeTypeFunctions()`
//! (`Analysis/src/ConstraintSolver.cpp:767-784`, hand-ported faithfully).

use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    reduce_type_functions_type_function::reduce_type_functions,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};
impl ConstraintSolver {
  pub fn constraint_solver_finalize_type_functions(&mut self) {
    // At this point, we've generalized. Let's try to finish reducing as much as we can, we'll leave warning to the typechecker
    let entries: Vec<(TypeId, *const Constraint)> = self
      .type_functions_to_finalize
      .iter()
      .map(|(t, constraint)| (*t, *constraint))
      .collect();

    for (t, constraint) in entries {
      let ty = follow_type_id(t);
      if !get_type_id::<TypeFunctionInstanceType>(ty).is_none() {
        let scope = unsafe { (*constraint).scope };
        let location = unsafe { (*constraint).location };

        let mut context = TypeFunctionContext::from_solver(
          NonNull::new(self as *mut ConstraintSolver).unwrap(),
          NonNull::new(scope).unwrap(),
          NonNull::new(constraint as *mut Constraint).unwrap(),
          NonNull::new(self.subtyping).unwrap(),
        );

        let result = reduce_type_functions(
          t,
          location,
          NonNull::new(&mut context as *mut TypeFunctionContext).unwrap(),
          true,
        );

        for r in result.reduced_types.iter() {
          self.unblock_type_id_location(*r, location);
        }
        for r in result.reduced_packs.iter() {
          self.unblock_type_pack_id_location(*r, location);
        }
      }
    }
  }
}
