use core::ptr::null_mut;

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault},
};

use crate::{
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, def_arena::DefArena,
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
    def_arena: *mut DefArena,
    key_arena: *mut RefinementKeyArena,
  ) -> Self {
    LUAU_ASSERT!(!def_arena.is_null());
    LUAU_ASSERT!(!key_arena.is_null());

    DataFlowGraphBuilder {
      graph: DataFlowGraphBuilder::empty(),
      def_arena,
      key_arena,
      handle: null_mut(),
      scopes: Default::default(),
      scope_stack: ScopeStack::new(),
      captures: DenseHashMap::new(Symbol::default()),
    }
  }
}
