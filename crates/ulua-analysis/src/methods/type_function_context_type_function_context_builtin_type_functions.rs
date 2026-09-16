//! C++ `TypeFunctionContext::TypeFunctionContext(NotNull<ConstraintSolver> cs,
//! NotNull<Scope> scope, NotNull<const Constraint> constraint,
//! NotNull<Subtyping> subtyping)` (BuiltinTypeFunctions.cpp:362-374). Pulls
//! arena/builtins/normalizer/runtime/ice/limits out of the constraint solver and
//! records the solver + constraint pointers.
use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::records::{
  constraint::Constraint, constraint_solver::ConstraintSolver, scope::Scope, subtyping::Subtyping,
  type_function_context::TypeFunctionContext,
};
impl TypeFunctionContext {
  /// C++ ctor used during constraint solving.
  pub fn from_solver(
    cs: NonNull<ConstraintSolver>,
    scope: NonNull<Scope>,
    constraint: NonNull<Constraint>,
    subtyping: NonNull<Subtyping>,
  ) -> Self {
    let cs_ref = unsafe { cs.as_ref() };

    TypeFunctionContext {
      // cs->arena / cs->builtinTypes / cs->normalizer / cs->typeFunctionRuntime
      // are raw owning pointers on the solver; wrap them as NonNull.
      arena: NonNull::new(cs_ref.arena).expect("ConstraintSolver::arena is null"),
      builtins: NonNull::new(cs_ref.builtin_types).expect("ConstraintSolver::builtinTypes is null"),
      scope,
      normalizer: NonNull::new(cs_ref.normalizer).expect("ConstraintSolver::normalizer is null"),
      type_function_runtime: NonNull::new(cs_ref.type_function_runtime)
        .expect("ConstraintSolver::typeFunctionRuntime is null"),
      // &cs->iceReporter and &cs->limits are addresses of by-value members.
      ice: NonNull::from(&cs_ref.ice_reporter),
      limits: NonNull::from(&cs_ref.limits),
      subtyping,
      solver: cs.as_ptr(),
      constraint: constraint.as_ptr(),
      user_func_name: None,
      fresh_instances: Vec::new(),
    }
  }
}
