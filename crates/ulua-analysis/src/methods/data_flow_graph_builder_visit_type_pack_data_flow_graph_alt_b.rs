use ulua_ast::records::ast_type_pack_explicit::AstTypePackExplicit;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `e` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_pack_ast_type_pack_explicit(&mut self, e: *mut AstTypePackExplicit) {
    unsafe {
      let type_list = (*e).type_list;
      self.visit_type_list(type_list);
    }
  }
}
