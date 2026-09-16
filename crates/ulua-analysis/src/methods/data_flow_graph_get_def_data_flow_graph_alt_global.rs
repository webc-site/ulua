//! C++ `DefId DataFlowGraph::getDef(const AstStatDeclareGlobal* global) const`
//! (`Analysis/src/DataFlowGraph.cpp:86`).
use ulua_ast::records::{ast_stat::AstStat, ast_stat_declare_global::AstStatDeclareGlobal};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::data_flow_graph::DataFlowGraph, type_aliases::def_id_def::DefId};

impl DataFlowGraph {
  pub fn get_def_declare_global(&self, global: *const AstStatDeclareGlobal) -> DefId {
    let def = self.declared_defs.find(&(global as *const AstStat));
    LUAU_ASSERT!(def.is_some());
    *def.unwrap()
  }
}
