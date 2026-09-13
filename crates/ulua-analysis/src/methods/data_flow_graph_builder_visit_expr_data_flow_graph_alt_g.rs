use alloc::string::String;
use core::{
  ffi::c_void,
  ptr::{NonNull, null},
};

use ulua_ast::records::{
  ast_expr_constant_string::AstExprConstantString, ast_expr_index_expr::AstExprIndexExpr,
  ast_node::AstNode,
};

use crate::{
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, def::Def,
    symbol::Symbol,
  },
  type_aliases::{def_id_def::DefId, def_id_refinement},
};
fn refinement_def_id(def: DefId) -> def_id_refinement::DefId {
  NonNull::new(def as *mut *const Def).unwrap()
}

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_index_expr(
    &mut self,
    i: *mut AstExprIndexExpr,
  ) -> DataFlowResult {
    unsafe {
      let parent = self.visit_expr_ast_expr((*i).expr);
      self.visit_expr_ast_expr((*i).index);

      let index_node = (*i).index as *mut AstNode;
      if (*index_node).is::<AstExprConstantString>() {
        let string = &*((*i).index as *mut AstExprConstantString);
        let index = String::from_utf8_lossy(string.value.as_bytes()).into_owned();

        let def = self.lookup_def_id_string_location(
          parent.def as *const Def,
          &index,
          (*i).base.base.location,
        );
        let key = (*self.key_arena).node(parent.parent, refinement_def_id(def), &index);
        return DataFlowResult {
          def: def as *const c_void,
          parent: key,
        };
      }

      let def = (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, true);
      DataFlowResult {
        def: def as *const c_void,
        parent: null(),
      }
    }
  }
}
