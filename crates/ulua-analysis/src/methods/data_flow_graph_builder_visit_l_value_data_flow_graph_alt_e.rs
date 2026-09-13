use alloc::string::String;

use ulua_ast::records::{
  ast_expr_constant_string::AstExprConstantString, ast_expr_index_expr::AstExprIndexExpr,
  ast_node::AstNode,
};

use crate::{
  functions::contains_subscripted_definition::contains_subscripted_definition,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, symbol::Symbol},
  type_aliases::def_id_def::DefId,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_ast_expr_index_expr_def_id(
    &mut self,
    i: *mut AstExprIndexExpr,
    incoming_def: DefId,
  ) -> DefId {
    unsafe {
      let parent_def = self.visit_expr_ast_expr((*i).expr).def as DefId;
      self.visit_expr_ast_expr((*i).index);

      let scope = self.current_scope();
      let index_node = (*i).index as *mut AstNode;
      if (*index_node).is::<AstExprConstantString>() {
        let string = &*((*i).index as *mut AstExprConstantString);
        let key = String::from_utf8_lossy(string.value.as_bytes()).into_owned();

        let subscripted = contains_subscripted_definition(incoming_def);
        let updated =
          (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, subscripted);
        (*scope)
          .props
          .get_or_insert(parent_def)
          .insert(key, updated);
        updated
      } else {
        let subscripted = true;
        (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, subscripted)
      }
    }
  }
}
