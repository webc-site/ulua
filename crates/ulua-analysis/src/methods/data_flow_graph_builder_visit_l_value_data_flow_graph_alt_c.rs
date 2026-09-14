use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::{
  functions::contains_subscripted_definition::contains_subscripted_definition,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_ast_expr_global_def_id(
    &mut self,
    g: *mut AstExprGlobal,
    incoming_def: DefId,
  ) -> DefId {
    unsafe {
      let scope = self.current_scope();
      let symbol = Symbol::from_global((*g).name);
      let subscripted = contains_subscripted_definition(incoming_def);

      let updated =
        (*self.def_arena).fresh_cell(symbol.clone(), (*g).base.base.location, subscripted);
      *(*scope).bindings.get_or_insert(symbol.clone()) = updated;
      self
        .captures
        .get_or_insert(symbol)
        .all_versions
        .push(updated);
      updated
    }
  }
}
