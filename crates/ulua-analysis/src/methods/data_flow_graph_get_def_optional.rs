use ulua_ast::records::ast_expr::AstExpr;

use crate::{records::data_flow_graph::DataFlowGraph, type_aliases::def_id_def::DefId};

impl DataFlowGraph {
  pub fn get_def_optional(&self, expr: *const AstExpr) -> Option<DefId> {
    // C++: auto def = astDefs.find(expr); if (!def) return nullopt; return NotNull{*def};
    self.ast_defs.find(&expr).map(|def| *def as DefId)
  }
}
