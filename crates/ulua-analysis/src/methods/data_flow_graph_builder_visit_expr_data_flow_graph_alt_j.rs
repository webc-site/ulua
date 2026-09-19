use core::{ffi::c_void, ptr::null};

use ulua_ast::records::ast_expr_unary::AstExprUnary;

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, symbol::Symbol,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `u` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_unary(&mut self, u: *mut AstExprUnary) -> DataFlowResult {
    unsafe {
      self.visit_expr_ast_expr((*u).expr);

      let def = (*self.def_arena).fresh_cell(Symbol::default(), (*u).base.base.location, false);
      DataFlowResult {
        def: def as *const c_void,
        parent: null(),
      }
    }
  }
}
