use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  data_flow_graph::DataFlowGraph, data_flow_graph_builder::DataFlowGraphBuilder,
};
impl DataFlowGraphBuilder {
  pub fn empty() -> DataFlowGraph {
    DataFlowGraph {
      ast_defs: DenseHashMap::default(),
      local_defs: DenseHashMap::default(),
      declared_defs: DenseHashMap::default(),
      def_to_symbol: DenseHashMap::default(),
      ast_refinement_keys: DenseHashMap::default(),
    }
  }
}
