use ulua_ast::records::location::Location;

use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes,
  internal_error_reporter::InternalErrorReporter, normalizer::Normalizer,
  overload_resolver::OverloadResolver, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
  type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
};

impl<'a> OverloadResolver<'a> {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约；其中 `limits` 必须非空，且其指向的
  /// `TypeCheckLimits`（对应 C++ 非拥有 `NotNull<TypeCheckLimits>`）不得早于返回的
  /// `OverloadResolver<'a>` 使用期被 drop 或被 `&mut` 独占改写——本函数只把它
  /// 转成共享借用 `&'a TypeCheckLimits` 存入，不取得所有权、不 clone。
  pub unsafe fn new(
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
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
      normalizer: Handle::from_ptr(normalizer),
      type_function_runtime: Handle::from_ptr(type_function_runtime),
      scope,
      ice: Handle::from_ptr(reporter),
      // C++ `NotNull<TypeCheckLimits> limits` 的非拥有直传；不再 clone，
      // 与上游共享同一 limits 对象（改写对原对象可见）。
      // Safety: 入参 limits 由调用方按 C++ NotNull 契约以非空指针传入，且其指向对象的
      // 存活期由 'a 覆盖整个 resolver 使用点；此处只借不降，不存在双重所有权。
      limits: unsafe { &*limits },
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
