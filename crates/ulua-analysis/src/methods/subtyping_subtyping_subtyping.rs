use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes,
  internal_error_reporter::InternalErrorReporter, normalizer::Normalizer, subtyping::Subtyping,
  type_arena::TypeArena, type_check_limits::TypeCheckLimits,
  type_function_runtime::TypeFunctionRuntime,
};

impl Subtyping {
  pub fn subtyping_not_null_builtin_types_not_null_type_arena_not_null_normalizer_not_null_type_function_runtime_not_null_internal_error_reporter(
    &mut self,
    builtin_types: Handle<BuiltinTypes>,
    type_arena: Handle<TypeArena>,
    normalizer: *mut Normalizer,
    type_function_runtime: *mut TypeFunctionRuntime,
    ice_reporter: *mut InternalErrorReporter,
  ) {
    self.builtin_types = builtin_types;
    self.arena = type_arena;
    self.normalizer = Handle::from_opt_ptr(normalizer);
    self.type_function_runtime = Handle::from_ptr(type_function_runtime);
    self.ice_reporter = Handle::from_ptr(ice_reporter);
  }

  /// C++ `Subtyping::Subtyping(NotNull<BuiltinTypes>, NotNull<TypeArena>,
  /// NotNull<Normalizer>, NotNull<TypeFunctionRuntime>,
  /// NotNull<InternalErrorReporter>)` — sets the five collaborators; every
  /// other member is default-constructed.
  pub fn subtyping_owned(
    builtin_types: Handle<BuiltinTypes>,
    type_arena: Handle<TypeArena>,
    normalizer: *mut Normalizer,
    type_function_runtime: *mut TypeFunctionRuntime,
    ice_reporter: *mut InternalErrorReporter,
  ) -> Self {
    Subtyping {
      builtin_types,
      arena: type_arena,
      normalizer: Handle::from_opt_ptr(normalizer),
      type_function_runtime: Handle::from_ptr(type_function_runtime),
      ice_reporter: Handle::from_ptr(ice_reporter),
      limits: TypeCheckLimits::default(),
      unique_types: null(),
      seen_types: DenseHashMap::default(),
      seen_packs: DenseHashMap::default(),
      result_cache: DenseHashMap::default(),
    }
  }
}
