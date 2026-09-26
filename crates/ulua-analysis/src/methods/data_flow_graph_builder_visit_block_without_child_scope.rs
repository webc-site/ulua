use ulua_ast::records::ast_stat_block::AstStatBlock;

use crate::{
  enums::control_flow::ControlFlow, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  /// cpp `visitBlockWithoutChildScope(AstStatBlock*)`：在当前作用域内逐条
  /// visit 块体语句，回吐首个非 None 控制流。
  pub fn visit_block_without_child_scope(&mut self, b: &AstStatBlock) -> ControlFlow {
    let mut first_control_flow: Option<ControlFlow> = None;

    for stat in b.body.iter_nodes() {
      let cf = self.visit_stat(stat);
      if cf != ControlFlow::None && first_control_flow.is_none() {
        first_control_flow = Some(cf);
      }
    }

    first_control_flow.unwrap_or(ControlFlow::None)
  }
}
