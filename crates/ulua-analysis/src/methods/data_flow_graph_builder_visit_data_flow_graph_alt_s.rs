use ulua_ast::records::{ast_stat::AstStat, ast_stat_declare_global::AstStatDeclareGlobal};

use crate::{
  enums::control_flow::ControlFlow,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
};

impl DataFlowGraphBuilder {
  pub(crate) fn visit_ast_stat_declare_global(
    &mut self,
    d: *mut AstStatDeclareGlobal,
  ) -> ControlFlow {
    unsafe {
      let symbol = Symbol::from_global((*d).name);
      let def = (*self.def_arena).fresh_cell(symbol.clone(), (*d).name_location, false);
      *self.graph.declared_defs.get_or_insert(d as *const AstStat) = def;
      *(*self.current_scope())
        .bindings
        .get_or_insert(symbol.clone()) = def;
      self.captures.get_or_insert(symbol).all_versions.push(def);

      self.visit_type_ast_type((*d).type_);
    }

    ControlFlow::None
  }
}
