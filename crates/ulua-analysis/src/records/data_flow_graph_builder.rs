use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    arena_handle::Handle, data_flow_graph::DataFlowGraph, def_arena::DefArena, dfg_scope::DfgScope,
    function_capture::FunctionCapture, internal_error_reporter::InternalErrorReporter,
    pinned_storage::PinnedStorage, refinement_key_arena::RefinementKeyArena, symbol::Symbol,
  },
  type_aliases::scope_stack::ScopeStack,
};
#[derive(Debug)]
pub struct DataFlowGraphBuilder {
  pub(crate) graph: DataFlowGraph,
  pub(crate) def_arena: Handle<DefArena>,
  pub(crate) key_arena: Handle<RefinementKeyArena>,
  /// C++ `struct InternalErrorReporter* handle = nullptr;` 的对应：可空成员
  /// 用 `Option<Handle<_>>` 表达（null 哨兵不进句柄），build() 期接线。
  pub(crate) handle: Option<Handle<InternalErrorReporter>>,
  pub(crate) scopes: PinnedStorage<DfgScope>,
  pub(crate) scope_stack: ScopeStack,
  pub(crate) captures: DenseHashMap<Symbol, FunctionCapture>,
}
