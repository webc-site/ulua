use ulua_ast::records::ast_type_error::AstTypeError;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `error` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_ast_type_error(&mut self, error: *mut AstTypeError) {
    unsafe {
      let types = (*error).types;
      for &t in types.as_slice() {
        self.visit_type_ast_type(t);
      }
    }
  }
}
