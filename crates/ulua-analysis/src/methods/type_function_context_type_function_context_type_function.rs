//! C++ `TypeFunctionContext::TypeFunctionContext(NotNull<TypeArena>,
//! NotNull<BuiltinTypes>, NotNull<Scope>, NotNull<Normalizer>,
//! NotNull<TypeFunctionRuntime>, NotNull<InternalErrorReporter>,
//! NotNull<TypeCheckLimits>, NotNull<Subtyping>)` (TypeFunction.h:60-81). Plain
//! field-init ctor; `solver`/`constraint` are null because this overload is
//! used when reducing outside of the constraint solver.
use alloc::vec::Vec;
use core::ptr::{NonNull, null, null_mut};

use crate::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
  normalizer::Normalizer, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
  type_check_limits::TypeCheckLimits, type_function_context::TypeFunctionContext,
  type_function_runtime::TypeFunctionRuntime,
};
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
