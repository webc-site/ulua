use core::{ffi::c_void, ptr::null};

use ulua_ast::records::ast_expr_interp_string::AstExprInterpString;

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult, symbol::Symbol,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_interp_string(
    &mut self,
    i: *mut AstExprInterpString,
  ) -> DataFlowResult {
    unsafe {
      for expr in (*i).expressions.iter() {
        self.visit_expr_ast_expr(*expr);
      }

      let def = (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, false);
      DataFlowResult {
        def: def as *const c_void,
        parent: null(),
      }
    }
  }
}
