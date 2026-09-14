//! Node: `cxx:Method:Luau.Analysis:Analysis/src/Subtyping.cpp:585:Subtyping`
//! Source: `Analysis/src/Subtyping.cpp:585-598` (hand-ported)

use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
  normalizer::Normalizer, subtyping::Subtyping, type_arena::TypeArena,
  type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
};
impl Subtyping {
  /// C++ `Subtyping::Subtyping(NotNull<BuiltinTypes>, NotNull<TypeArena>,
  /// NotNull<Normalizer>, NotNull<TypeFunctionRuntime>,
  /// NotNull<InternalErrorReporter>)` — sets the five collaborators; every
  /// other member is default-constructed.
  pub fn subtyping_owned(
    builtin_types: *mut BuiltinTypes,
    type_arena: *mut TypeArena,
    normalizer: *mut Normalizer,
    type_function_runtime: *mut TypeFunctionRuntime,
    ice_reporter: *mut InternalErrorReporter,
  ) -> Self {
    Subtyping {
      builtin_types,
      arena: type_arena,
      normalizer,
      type_function_runtime,
      ice_reporter,
      limits: TypeCheckLimits::default(),
      unique_types: null(),
      seen_types: DenseHashMap::new((null(), null())),
      seen_packs: DenseHashMap::new((null(), null())),
      result_cache: DenseHashMap::new((null(), null())),
    }
  }
}
