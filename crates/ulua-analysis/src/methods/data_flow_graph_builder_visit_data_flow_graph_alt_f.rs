use ulua_ast::records::ast_stat_break::AstStatBreak;

use crate::{
  enums::control_flow::ControlFlow, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  pub fn visit_ast_stat_break(&mut self, _b: *mut AstStatBreak) -> ControlFlow {
    ControlFlow::Breaks
  }
}
