use alloc::vec::Vec;

use ulua_ast::records::{
  ast_expr_table::AstExprTable, ast_node::AstNode, ast_stat_local::AstStatLocal,
};

use crate::{
  enums::control_flow::ControlFlow,
  functions::contains_subscripted_definition::contains_subscripted_definition,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `l` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local(&mut self, l: *mut AstStatLocal) -> ControlFlow {
    unsafe {
      let mut defs: Vec<DefId> = Vec::with_capacity((*l).values.size);
      for expr in (*l).values.iter() {
        defs.push(self.visit_expr_ast_expr(*expr).def as DefId);
      }

      for (i, local) in (*l).vars.iter().enumerate() {
        let local = *local;
        if !(*local).annotation.is_null() {
          self.visit_type_ast_type((*local).annotation);
        }

        let subscripted = i < defs.len() && contains_subscripted_definition(defs[i]);
        let mut def =
          (*self.def_arena).fresh_cell(Symbol::from_local(local), (*local).location, subscripted);

        if i < (*l).values.size {
          let expr = *(*l).values.data.add(i);
          if (*(expr as *mut AstNode)).is::<AstExprTable>() {
            def = defs[i];
          }
        }

        *self.graph.local_defs.get_or_insert(local as *const _) = def;
        *(*self.current_scope())
          .bindings
          .get_or_insert(Symbol::from_local(local)) = def;
        self
          .captures
          .get_or_insert(Symbol::from_local(local))
          .all_versions
          .push(def);
      }
    }

    ControlFlow::None
  }
}
