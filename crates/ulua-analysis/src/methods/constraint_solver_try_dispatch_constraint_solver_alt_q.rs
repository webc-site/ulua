//! `bool ConstraintSolver::tryDispatch(const ReducePackConstraint& c, NotNull<const Constraint> constraint, bool force)`
//! (`Analysis/src/ConstraintSolver.cpp:2935-2972`, hand-ported faithfully).

use core::{ffi::c_void, ptr::NonNull};

use crate::{
  functions::{
    follow_type_pack::follow_type_pack_id, get_error::get_type_error,
    reduce_type_functions_type_function_alt_b::reduce_type_functions,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    reduce_pack_constraint::ReducePackConstraint, type_function_context::TypeFunctionContext,
    uninhabited_type_function::UninhabitedTypeFunction,
    uninhabited_type_pack_function::UninhabitedTypePackFunction,
  },
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_reduce_pack_constraint_not_null_constraint_bool(
    &mut self,
    c: &ReducePackConstraint,
    constraint: *const Constraint,
    force: bool,
  ) -> bool {
    let tp = unsafe { follow_type_pack_id(c.tp) };

    let scope = unsafe { (*constraint).scope };
    let location = unsafe { (*constraint).location };

    let mut context = TypeFunctionContext::from_solver(
      NonNull::new(self as *mut ConstraintSolver).unwrap(),
      NonNull::new(scope).unwrap(),
      NonNull::new(constraint as *mut Constraint).unwrap(),
      NonNull::new(self.subtyping).unwrap(),
    );
    let result = reduce_type_functions(
      tp,
      location,
      NonNull::new(&mut context as *mut TypeFunctionContext).unwrap(),
      force,
    );

    for r in result.reduced_types.iter() {
      self.unblock_type_id_location(*r, location);
    }

    for r in result.reduced_packs.iter() {
      self.unblock_type_pack_id_location(*r, location);
    }

    let reduction_finished = result.blocked_types.empty() && result.blocked_packs.empty();

    if force || reduction_finished {
      // if we're completely dispatching this constraint, we want to record any uninhabited type functions to unblock.
      for error in result.errors.iter() {
        if let Some(utf) = get_type_error::<UninhabitedTypeFunction>(error) {
          self
            .uninhabited_type_functions
            .insert(utf.ty as *const c_void);
        } else if let Some(utpf) = get_type_error::<UninhabitedTypePackFunction>(error) {
          self
            .uninhabited_type_functions
            .insert(utpf.tp as *const c_void);
        }
      }
    }

    if force {
      return true;
    }

    for b in result.blocked_types.iter() {
      self.block_type_id_not_null_constraint(*b, constraint);
    }

    for b in result.blocked_packs.iter() {
      self.block_type_pack_id_not_null_constraint(*b, constraint);
    }

    reduction_finished
  }
}
