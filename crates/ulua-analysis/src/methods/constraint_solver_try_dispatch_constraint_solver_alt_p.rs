//! `bool ConstraintSolver::tryDispatch(const ReduceConstraint& c, NotNull<const Constraint> constraint, bool force)`
//! (`Analysis/src/ConstraintSolver.cpp:2879-2933`, hand-ported faithfully).

use core::{ffi::c_void, mem::take, ptr::NonNull};

use crate::{
  functions::{
    follow_type::follow_type_id, get_error::get_type_error, get_type_alt_j::get_type_id,
    reduce_type_functions_type_function::reduce_type_functions,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    reduce_constraint::ReduceConstraint, type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    uninhabited_type_function::UninhabitedTypeFunction,
    uninhabited_type_pack_function::UninhabitedTypePackFunction,
  },
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_reduce_constraint_not_null_constraint_bool(
    &mut self,
    c: &ReduceConstraint,
    constraint: *const Constraint,
    force: bool,
  ) -> bool {
    let mut ty = follow_type_id(c.ty);

    let scope = unsafe { (*constraint).scope };
    let location = unsafe { (*constraint).location };

    let mut context = TypeFunctionContext::from_solver(
      NonNull::new(self as *mut ConstraintSolver).unwrap(),
      NonNull::new(scope).unwrap(),
      NonNull::new(constraint as *mut Constraint).unwrap(),
      NonNull::new(self.subtyping).unwrap(),
    );
    let mut result = reduce_type_functions(
      ty,
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

    for ity in result.irreducible_types.iter() {
      self
        .uninhabited_type_functions
        .insert(*ity as *const c_void);
      self.unblock_type_id_location(*ity, location);
    }

    let reduction_finished = result.blocked_types.empty() && result.blocked_packs.empty();

    ty = follow_type_id(ty);

    // If we couldn't reduce this type function, stick it in the set!
    if get_type_id::<TypeFunctionInstanceType>(ty).is_some()
      && result.irreducible_types.find(&ty).is_none()
    {
      *self.type_functions_to_finalize.get_or_insert(ty) = constraint;
    }

    if force || reduction_finished {
      for message in take(&mut result.messages) {
        self.report_error_type_error(message);
      }

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
