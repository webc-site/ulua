use ulua_ast::records::ast_type_pack_variadic::AstTypePackVariadic;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `v` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_pack_ast_type_pack_variadic(&mut self, v: *mut AstTypePackVariadic) {
    unsafe {
      self.visit_type_ast_type((*v).variadic_type);
    }
  }
}
