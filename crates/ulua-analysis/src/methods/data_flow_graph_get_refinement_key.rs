use core::ptr::null;

use ulua_ast::records::ast_expr::AstExpr;

use crate::records::{data_flow_graph::DataFlowGraph, refinement_key::RefinementKey};
impl DataFlowGraph {
  pub fn get_refinement_key(&self, expr: *const AstExpr) -> *const RefinementKey {
    if let Some(v) = self.ast_refinement_keys.find(&expr) {
      *v
    } else {
      null()
    }
  }
}
