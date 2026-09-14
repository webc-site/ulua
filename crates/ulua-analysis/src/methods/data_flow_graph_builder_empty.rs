use core::ptr::null;

use ulua_ast::records::{ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  data_flow_graph::DataFlowGraph, data_flow_graph_builder::DataFlowGraphBuilder, def::Def,
};
impl DataFlowGraphBuilder {
  pub fn empty() -> DataFlowGraph {
    DataFlowGraph {
      ast_defs: DenseHashMap::new(null::<AstExpr>()),
      local_defs: DenseHashMap::new(null::<AstLocal>()),
      declared_defs: DenseHashMap::new(null::<AstStat>()),
      def_to_symbol: DenseHashMap::new(null::<Def>()),
      ast_refinement_keys: DenseHashMap::new(null::<AstExpr>()),
    }
  }
}
