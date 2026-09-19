use ulua_ast::records::ast_expr_group::AstExprGroup;

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `group` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_group(&mut self, group: *mut AstExprGroup) -> DataFlowResult {
    let expr = unsafe { (*group).expr };
    self.visit_expr_ast_expr(expr)
  }
}
