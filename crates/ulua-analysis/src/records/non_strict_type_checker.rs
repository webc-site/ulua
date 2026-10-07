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
pub struct NonStrictTypeChecker<'a> {
  // 句柄化：原 C++ NotNull 裸指针，目标为宿主（TypeCheckSharedState/Frontend）
  // 独占持有的进程级单例。
  pub builtin_types: Handle<BuiltinTypes>,
  pub type_function_runtime: Handle<TypeFunctionRuntime>,
  pub ice: Handle<InternalErrorReporter>,
  pub arena: Handle<TypeArena>,
  pub module: *mut Module,
  pub normalizer: Normalizer,
  pub subtyping: Subtyping,
  // 句柄化：构造期由 `&DataFlowGraph`（C++ `const DataFlowGraph&` 形参）经
  // `Handle::from_ref` 接线；目标为 TypeChecker2 在进入 non-strict 检查前
  // 构建完成、整会话存活的 DFG，checker 只读消费（`dfg_ref` 统一解引用）。
  pub dfg: Handle<DataFlowGraph>,
  // 与 `TypeChecker2::stack` 同步句柄化（共用 `StackPusher` 守卫，#24 b13-tc2-fields）。
  pub stack: Vec<Handle<Scope>>,
  pub cached_negations: DenseHashMap<TypeId, TypeId>,
  // C++ `NotNull<TypeCheckLimits>`：checker 全程只读消费，引用 + 生命周期直接
  // 承载非空与存活证明（对齐 `OverloadResolver::limits` 形状），不再把共享借用
  // 转铸成 `*mut`。
  pub limits: &'a TypeCheckLimits,
  /// C++ `int nonStrictRecursionCount = 0;` (private member of `NonStrictTypeChecker`).
  pub non_strict_recursion_count: i32,
}
