//! Source: `Analysis/src/NonStrictTypeChecker.cpp` (hand-ported; fields only)

use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, data_flow_graph::DataFlowGraph,
    internal_error_reporter::InternalErrorReporter, module::Module, normalizer::Normalizer,
    scope::Scope, subtyping::Subtyping, type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_id::TypeId,
};
#[derive(Debug)]
pub struct NonStrictTypeChecker {
  // 句柄化：原 C++ NotNull 裸指针，目标为宿主（TypeCheckSharedState/Frontend）
  // 独占持有的进程级单例。
  pub builtin_types: Handle<BuiltinTypes>,
  pub type_function_runtime: Handle<TypeFunctionRuntime>,
  pub ice: Handle<InternalErrorReporter>,
  pub arena: Handle<TypeArena>,
  pub module: *mut Module,
  pub normalizer: Normalizer,
  pub subtyping: Subtyping,
  pub dfg: *const DataFlowGraph,
  // 与 `TypeChecker2::stack` 同步句柄化（共用 `StackPusher` 守卫，#24 b13-tc2-fields）。
  pub stack: Vec<Handle<Scope>>,
  pub cached_negations: DenseHashMap<TypeId, TypeId>,
  pub limits: *mut TypeCheckLimits,
  /// C++ `int nonStrictRecursionCount = 0;` (private member of `NonStrictTypeChecker`).
  pub non_strict_recursion_count: i32,
}
