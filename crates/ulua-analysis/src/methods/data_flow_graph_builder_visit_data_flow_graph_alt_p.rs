use ulua_ast::records::{ast_expr::AstExpr, ast_stat_local_function::AstStatLocalFunction};

use crate::{
  enums::control_flow::ControlFlow,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
};

impl DataFlowGraphBuilder {
  pub(crate) fn visit_ast_stat_local_function(
    &mut self,
    l: *mut AstStatLocalFunction,
  ) -> ControlFlow {
    unsafe {
      let symbol = Symbol::from_local((*l).name);
      let def = (*self.def_arena).fresh_cell(symbol.clone(), (*l).base.base.location, false);
      *self.graph.local_defs.get_or_insert((*l).name as *const _) = def;
      *(*self.current_scope())
        .bindings
        .get_or_insert(symbol.clone()) = def;
      self.captures.get_or_insert(symbol).all_versions.push(def);

      self.visit_expr_ast_expr((*l).func as *mut AstExpr);
    }

    ControlFlow::None
  }
}
