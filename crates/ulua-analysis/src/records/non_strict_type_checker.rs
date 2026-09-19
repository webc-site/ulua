//! Source: `Analysis/src/NonStrictTypeChecker.cpp` (hand-ported; fields only)

use alloc::vec::Vec;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  records::{
    builtin_types::BuiltinTypes, data_flow_graph::DataFlowGraph,
    internal_error_reporter::InternalErrorReporter, module::Module, normalizer::Normalizer,
    scope::Scope, subtyping::Subtyping, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_id::TypeId,
};
#[derive(Debug)]
pub struct NonStrictTypeChecker {
  pub builtin_types: *mut BuiltinTypes,
  pub type_function_runtime: *mut TypeFunctionRuntime,
  pub ice: *mut InternalErrorReporter,
  pub arena: *mut TypeArena,
  pub module: *mut Module,
  pub normalizer: Normalizer,
  pub subtyping: Subtyping,
  pub dfg: *const DataFlowGraph,
  pub no_type_function_errors: DenseHashSet<TypeId>,
  pub stack: Vec<*mut Scope>,
  pub cached_negations: DenseHashMap<TypeId, TypeId>,
  pub limits: *mut TypeCheckLimits,
  /// C++ `int nonStrictRecursionCount = 0;` (private member of `NonStrictTypeChecker`).
  pub non_strict_recursion_count: i32,
}
