//! C++ `DefId DataFlowGraph::getDef(const AstLocal* local) const`
//! (`Analysis/src/DataFlowGraph.cpp:79`).
use ulua_ast::records::ast_local::AstLocal;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::data_flow_graph::DataFlowGraph, type_aliases::def_id_def::DefId};

impl DataFlowGraph {
  pub fn get_def_local(&self, local: *const AstLocal) -> DefId {
    let def = self.local_defs.find(&local);
    LUAU_ASSERT!(def.is_some());
    *def.unwrap()
  }
}
