use core::{ffi::c_void, ptr::null};

use ulua_ast::records::ast_expr_if_else::AstExprIfElse;

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, symbol::Symbol,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `i` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_if_else(&mut self, i: *mut AstExprIfElse) -> DataFlowResult {
    unsafe {
      self.visit_expr_ast_expr((*i).condition);
      self.visit_expr_ast_expr((*i).true_expr);
      self.visit_expr_ast_expr((*i).false_expr);

      let def = (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, false);

      DataFlowResult {
        def: def as *const c_void,
        parent: null(),
      }
    }
  }
}
