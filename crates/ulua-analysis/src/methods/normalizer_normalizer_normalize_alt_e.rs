use alloc::collections::BTreeMap;
use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, normalizer::Normalizer, type_arena::TypeArena,
    unifier_shared_state::UnifierSharedState,
  },
};
impl Normalizer {
  /// C++ `Normalizer::Normalizer(TypeArena*, NotNull<BuiltinTypes>,
  /// NotNull<UnifierSharedState>, SolverMode, bool cacheInhabitance = false)`
  /// (`Analysis/src/Normalize.cpp:858`). Owned constructor: the five
  /// collaborators are stored; every cache is default-empty (the
  /// `DenseHashMap`s take their `nullptr` empty-key, matching the C++
  /// member initializers `{nullptr}` / `{{nullptr, nullptr}}`).
  pub fn new(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    shared_state: *mut UnifierSharedState,
    solver_mode: SolverMode,
    cache_inhabitance: bool,
  ) -> Self {
    Normalizer {
      cached_normals: BTreeMap::new(),
      cached_intersections: BTreeMap::new(),
      cached_unions: BTreeMap::new(),
      cached_type_ids: BTreeMap::new(),
      cached_is_inhabited: DenseHashMap::new(null()),
      cached_is_inhabited_intersection: DenseHashMap::new((null(), null())),
      fuel: None,
      arena,
      builtin_types,
      shared_state,
      cache_inhabitance,
      solver_mode,
    }
  }

  pub fn normalizer_type_arena_not_null_builtin_types_not_null_unifier_shared_state_solver_mode_bool(
    &mut self,
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    shared_state: *mut UnifierSharedState,
    solver_mode: SolverMode,
    cache_inhabitance: bool,
  ) {
    self.arena = arena;
    self.builtin_types = builtin_types;
    self.shared_state = shared_state;
    self.cache_inhabitance = cache_inhabitance;
    self.solver_mode = solver_mode;
  }
}
