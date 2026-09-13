use ulua_ast::records::ast_type_reference::AstTypeReference;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `r` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_ast_type_reference(&mut self, r: *mut AstTypeReference) {
    let parameters = unsafe { (*r).parameters };
    for param in parameters.as_slice() {
      if !param.r#type.is_null() {
        self.visit_type_ast_type(param.r#type);
      } else {
        self.visit_type_pack_ast_type_pack(param.type_pack);
      }
    }
  }
}
