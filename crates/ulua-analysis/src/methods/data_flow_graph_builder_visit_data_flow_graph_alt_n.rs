use ulua_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;

use crate::{
  enums::control_flow::ControlFlow, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    c: *mut AstStatCompoundAssign,
  ) -> ControlFlow {
    unsafe {
      let c = &*c;
      let value = c.value;
      let var = c.var;

      let _ = self.visit_expr_ast_expr(value);
      let _ = self.visit_expr_ast_expr(var);
    }

    ControlFlow::None
  }
}
