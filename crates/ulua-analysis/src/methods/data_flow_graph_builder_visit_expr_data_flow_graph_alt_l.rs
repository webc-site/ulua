use ulua_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

use crate::records::{
  data_flow_graph_builder::DataFlowGraphBuilder, data_flow_result::DataFlowResult,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_expr_ast_expr_type_assertion(
    &mut self,
    t: *mut AstExprTypeAssertion,
  ) -> DataFlowResult {
    let expr = unsafe { (*t).expr };
    let annotation = unsafe { (*t).annotation };
    let def = self.visit_expr_ast_expr(expr);
    self.visit_type_ast_type(annotation);
    def
  }
}
