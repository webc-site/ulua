use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::{
  records::{
    arena_handle::Handle, data_flow_graph_builder::DataFlowGraphBuilder, def_arena::DefArena,
    function_capture::FunctionCapture, refinement_key_arena::RefinementKeyArena, symbol::Symbol,
  },
  type_aliases::scope_stack::ScopeStack,
};
impl DenseDefault for FunctionCapture {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl DenseDefault for Symbol {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl DataFlowGraphBuilder {
  pub fn data_flow_graph_builder_not_null_def_arena_not_null_refinement_key_arena(
    def_arena: Handle<DefArena>,
    key_arena: Handle<RefinementKeyArena>,
  ) -> Self {
    DataFlowGraphBuilder {
      graph: DataFlowGraphBuilder::empty(),
      // C++ `NotNull` 契约由 Handle（NonNull 编码非空）在类型层承接，
      // 原判空断言的不空前提按构造成立。
      def_arena,
      key_arena,
      // C++ 成员 `InternalErrorReporter* handle = nullptr;` 的对应默认值。
      handle: None,
      scopes: Default::default(),
      scope_stack: ScopeStack::new(),
      captures: DenseHashMap::new(Symbol::default()),
    }
  }
}
