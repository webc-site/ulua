//! `type_function_context` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::{NonNull, null, null_mut};

use ulua_ast::records::{location::Location, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::{alias, alias_ref},
    builtin_types::BuiltinTypes,
    constraint::Constraint,
    constraint_solver::ConstraintSolver,
    internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer,
    scope::Scope,
    subtyping::Subtyping,
    type_arena::TypeArena,
    type_check_limits::TypeCheckLimits,
    type_function_context::TypeFunctionContext,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::constraint_v::ConstraintV,
};

// C++ `NotNull<Constraint> TypeFunctionContext::pushConstraint(ConstraintV&& c)
// const` (BuiltinTypeFunctions.cpp:376-387). Forwards to the solver and, when a
// current constraint exists, inherits its blocks onto the new constraint.

impl TypeFunctionContext {
  pub fn push_constraint(&self, c: ConstraintV) -> NonNull<Constraint> {
    LUAU_ASSERT!(!self.solver.is_null());

    let location = if !self.constraint.is_null() {
      alias_ref(self.constraint).location
    } else {
      Location::new(
        Position { line: 0, column: 0 },
        Position { line: 0, column: 0 },
      )
    };

    let new_constraint = alias(self.solver).push_constraint(self.scope, location, c);

    // Every constraint that is blocked on the current constraint must also be
    // blocked on this new one.
    if !self.constraint.is_null() {
      alias(self.solver).inherit_blocks(
        self.constraint,
        new_constraint.as_ptr() as *const Constraint,
      );
    }

    new_constraint
  }
}

// C++ `TypeFunctionContext::TypeFunctionContext(NotNull<ConstraintSolver> cs,
// NotNull<Scope> scope, NotNull<const Constraint> constraint,
// NotNull<Subtyping> subtyping)` (BuiltinTypeFunctions.cpp:362-374). Pulls
// arena/builtins/normalizer/runtime/ice/limits out of the constraint solver and
// records the solver + constraint pointers.

impl TypeFunctionContext {
  /// C++ ctor used during constraint solving.
  pub fn from_solver(
    cs: NonNull<ConstraintSolver>,
    scope: NonNull<Scope>,
    constraint: NonNull<Constraint>,
    subtyping: NonNull<Subtyping>,
  ) -> Self {
    let cs_ref = alias_ref(cs.as_ptr());

    TypeFunctionContext {
      // cs->arena / cs->builtinTypes / cs->normalizer / cs->typeFunctionRuntime
      // are raw owning pointers on the solver; wrap them as NonNull.
      arena: NonNull::new(cs_ref.arena.as_ptr()).expect("ConstraintSolver::arena is null"),
      builtins: NonNull::new(cs_ref.builtin_types.as_ptr())
        .expect("ConstraintSolver::builtinTypes is null"),
      scope,
      normalizer: NonNull::new(cs_ref.normalizer.as_ptr())
        .expect("ConstraintSolver::normalizer is null"),
      type_function_runtime: NonNull::new(cs_ref.type_function_runtime.as_ptr())
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

// C++ `TypeFunctionContext::TypeFunctionContext(NotNull<TypeArena>,
// NotNull<BuiltinTypes>, NotNull<Scope>, NotNull<Normalizer>,
// NotNull<TypeFunctionRuntime>, NotNull<InternalErrorReporter>,
// NotNull<TypeCheckLimits>, NotNull<Subtyping>)` (TypeFunction.h:60-81). Plain
// field-init ctor; `solver`/`constraint` are null because this overload is
// used when reducing outside of the constraint solver.

impl TypeFunctionContext {
  /// C++ ctor used when reducing outside of the constraint solver.
  pub fn from_components(
    arena: NonNull<TypeArena>,
    builtins: NonNull<BuiltinTypes>,
    scope: NonNull<Scope>,
    normalizer: NonNull<Normalizer>,
    type_function_runtime: NonNull<TypeFunctionRuntime>,
    ice: NonNull<InternalErrorReporter>,
    limits: NonNull<TypeCheckLimits>,
    subtyping: NonNull<Subtyping>,
  ) -> Self {
    TypeFunctionContext {
      arena,
      builtins,
      scope,
      normalizer,
      type_function_runtime,
      ice,
      limits,
      subtyping,
      solver: null_mut(),
      constraint: null(),
      user_func_name: None,
      fresh_instances: Vec::new(),
    }
  }
}
