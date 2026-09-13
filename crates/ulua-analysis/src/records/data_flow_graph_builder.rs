use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    data_flow_graph::DataFlowGraph, def_arena::DefArena, dfg_scope::DfgScope,
    function_capture::FunctionCapture, internal_error_reporter::InternalErrorReporter,
    pinned_storage::PinnedStorage, refinement_key_arena::RefinementKeyArena, symbol::Symbol,
  },
  type_aliases::scope_stack::ScopeStack,
};
#[derive(Debug)]
pub struct DataFlowGraphBuilder {
  pub(crate) graph: DataFlowGraph,
  pub(crate) def_arena: *mut DefArena,
  pub(crate) key_arena: *mut RefinementKeyArena,
  pub(crate) handle: *mut InternalErrorReporter,
  pub(crate) scopes: PinnedStorage<DfgScope>,
  pub(crate) scope_stack: ScopeStack,
  pub(crate) captures: DenseHashMap<Symbol, FunctionCapture>,
}
