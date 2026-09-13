use core::{
  ffi::c_void,
  ptr::{NonNull, null_mut},
};

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  functions::push_type_into::push_type_into,
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    internal_error_reporter::InternalErrorReporter, push_type_constraint::PushTypeConstraint,
    subtyping::Subtyping, unifier_2::Unifier2,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_push_type_constraint_not_null_constraint_bool(
    &mut self,
    c: &PushTypeConstraint,
    constraint: *const Constraint,
    force: bool,
  ) -> bool {
    let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
            NonNull::new(self.arena).unwrap(),
            NonNull::new(self.builtin_types).unwrap(),
            NonNull::new(unsafe { (*constraint).scope }).unwrap(),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).unwrap(),
            &mut self.uninhabited_type_functions as *mut DenseHashSet<*const c_void>,
        );

    let mut subtyping = Subtyping::subtyping_owned(
      self.builtin_types,
      self.arena,
      self.normalizer,
      self.type_function_runtime,
      &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter,
    );

    // NOTE: If we don't do this check up front, we almost immediately start
    // spawning tons of push type constraints. It's pretty important.
    if self.is_blocked_type_id(c.expected_type) {
      self.block_type_id_not_null_constraint(c.expected_type, constraint);
      // If we're forcing this constraint and the expected type is blocked, we
      // should just bail.
      return force;
    }

    let mut empty: DenseHashSet<*const c_void> = DenseHashSet::new(null_mut());
    let result = push_type_into(
      NonNull::new(c.ast_types as *mut DenseHashMap<*const AstExpr, TypeId>).unwrap(),
      NonNull::new(c.ast_expected_types as *mut DenseHashMap<*const AstExpr, TypeId>).unwrap(),
      NonNull::new(self as *mut ConstraintSolver).unwrap(),
      NonNull::new(constraint as *mut Constraint).unwrap(),
      NonNull::new(&mut empty as *mut DenseHashSet<*const c_void>).unwrap(),
      NonNull::new(&mut u2 as *mut Unifier2).unwrap(),
      NonNull::new(&mut subtyping as *mut Subtyping).unwrap(),
      c.expected_type,
      c.expr,
    );

    // If we're forcing this constraint, just early exit: we can continue
    // inferring the rest of the file, we might just error when we shouldn't.
    if force || result.incomplete_types.is_empty() {
      return true;
    }

    for incomplete in &result.incomplete_types {
      let addition = self.push_constraint(
        NonNull::new(unsafe { (*constraint).scope }).unwrap(),
        unsafe { (*constraint).location },
        ConstraintV::PushType(PushTypeConstraint {
          expected_type: incomplete.expected_type,
          target_type: incomplete.target_type,
          ast_types: c.ast_types,
          ast_expected_types: c.ast_expected_types,
          expr: incomplete.expr,
        }),
      );
      self.inherit_blocks(constraint, addition.as_ptr());
    }

    true
  }
}
