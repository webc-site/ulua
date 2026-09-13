use ulua_ast::records::location::Location;

use crate::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
  normalizer::Normalizer, overload_resolver::OverloadResolver, scope::Scope, subtyping::Subtyping,
  type_arena::TypeArena, type_check_limits::TypeCheckLimits,
  type_function_runtime::TypeFunctionRuntime,
};

impl OverloadResolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn new(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    normalizer: *mut Normalizer,
    type_function_runtime: *mut TypeFunctionRuntime,
    scope: *mut Scope,
    reporter: *mut InternalErrorReporter,
    limits: *mut TypeCheckLimits,
    call_location: Location,
  ) -> Self {
    Self {
      builtin_types,
      arena,
      normalizer,
      type_function_runtime,
      scope,
      ice: reporter,
      limits: unsafe { (*limits).clone() },
      subtyping: Subtyping::subtyping_owned(
        builtin_types,
        arena,
        normalizer,
        type_function_runtime,
        reporter,
      ),
      call_loc: call_location,
    }
  }
}
