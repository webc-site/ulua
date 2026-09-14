use ulua_ast::records::ast_type_typeof::AstTypeTypeof;

use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_type_ast_type_typeof(&mut self, t: *mut AstTypeTypeof) {
    unsafe {
      self.visit_expr_ast_expr((*t).expr);
    }
  }
}
