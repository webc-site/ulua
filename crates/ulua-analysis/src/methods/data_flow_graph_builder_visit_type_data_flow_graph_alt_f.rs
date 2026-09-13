use ulua_ast::records::ast_type_union::AstTypeUnion;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `u` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_ast_type_union(&mut self, u: *mut AstTypeUnion) {
    unsafe {
      let types = (*u).types;
      for &t in types.as_slice() {
        self.visit_type_ast_type(t);
      }
    }
  }
}
