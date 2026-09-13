use core::ptr::null;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
  ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
  ast_expr_local::AstExprLocal, ast_node::AstNode,
};
use ulua_common::LUAU_ASSERT;

use crate::{
  records::data_flow_graph_builder::DataFlowGraphBuilder, type_aliases::def_id_def::DefId,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `e` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_l_value_ast_expr_def_id(&mut self, e: *mut AstExpr, incoming_def: DefId) {
    unsafe {
      let node = e as *mut AstNode;
      let def = if (*node).is::<AstExprLocal>() {
        self.visit_l_value_ast_expr_local_def_id(e as *mut AstExprLocal, incoming_def)
      } else if (*node).is::<AstExprGlobal>() {
        self.visit_l_value_ast_expr_global_def_id(e as *mut AstExprGlobal, incoming_def)
      } else if (*node).is::<AstExprIndexName>() {
        self.visit_l_value_ast_expr_index_name_def_id(e as *mut AstExprIndexName, incoming_def)
      } else if (*node).is::<AstExprIndexExpr>() {
        self.visit_l_value_ast_expr_index_expr_def_id(e as *mut AstExprIndexExpr, incoming_def)
      } else if (*node).is::<AstExprError>() {
        self.visit_l_value_ast_expr_error_def_id(e as *mut AstExprError, incoming_def)
      } else {
        LUAU_ASSERT!(false);
        null()
      };

      *self.graph.ast_defs.get_or_insert(e as *const AstExpr) = def;
    }
  }
}
