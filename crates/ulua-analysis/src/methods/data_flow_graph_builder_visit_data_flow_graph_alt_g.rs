use ulua_ast::records::ast_stat_continue::AstStatContinue;

use crate::{
  enums::control_flow::ControlFlow, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  pub fn visit_ast_stat_continue(&mut self, _c: *mut AstStatContinue) -> ControlFlow {
    ControlFlow::Continues
  }
}
