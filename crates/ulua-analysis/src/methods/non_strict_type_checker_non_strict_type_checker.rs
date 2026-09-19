use core::ptr::{null, null_mut};

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  enums::solver_mode::SolverMode,
  records::{
    builtin_types::BuiltinTypes, data_flow_graph::DataFlowGraph,
    internal_error_reporter::InternalErrorReporter, module::Module,
    non_strict_type_checker::NonStrictTypeChecker, normalizer::Normalizer, subtyping::Subtyping,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
};
impl NonStrictTypeChecker {
  pub fn new(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    type_function_runtime: *mut TypeFunctionRuntime,
    ice: *const InternalErrorReporter,
    unifier_state: *mut UnifierSharedState,
    dfg: *const DataFlowGraph,
    limits: *const TypeCheckLimits,
    module: *mut Module,
  ) -> Self {
    let normalizer = Normalizer::new(arena, builtin_types, unifier_state, SolverMode::New, true);
    let subtyping = Subtyping::subtyping_owned(
      builtin_types,
      arena,
      null_mut(),
      type_function_runtime,
      ice as *mut InternalErrorReporter,
    );

    NonStrictTypeChecker {
      builtin_types,
      type_function_runtime,
      ice: ice as *mut InternalErrorReporter,
      arena,
      module,
      normalizer,
      subtyping,
      dfg,
      no_type_function_errors: DenseHashSet::new(null()),
      stack: Vec::new(),
      cached_negations: DenseHashMap::new(null()),
      limits: limits as *mut TypeCheckLimits,
      non_strict_recursion_count: 0,
    }
  }

  /// Wires `subtyping.normalizer` to the embedded `normalizer` after this
  /// checker has moved into its final stack slot.
  ///
  /// # Safety
  /// The checker must not be moved after this call.
  pub unsafe fn wire_self_pointers(&mut self) {
    self.subtyping.normalizer = &mut self.normalizer as *mut Normalizer;
  }
}
