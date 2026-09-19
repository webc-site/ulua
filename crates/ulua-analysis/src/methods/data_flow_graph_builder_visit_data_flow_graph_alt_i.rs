use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_node::AstNode, ast_stat_expr::AstStatExpr},
  rtti::ast_node_as,
};

use crate::{
  enums::control_flow::ControlFlow, functions::does_call_error::does_call_error,
  records::data_flow_graph_builder::DataFlowGraphBuilder,
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `e` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_expr(&mut self, e: *mut AstStatExpr) -> ControlFlow {
    unsafe {
      let e = &*e;
      self.visit_expr_ast_expr(e.expr);

      let call = ast_node_as::<AstExprCall>(e.expr as *mut AstNode);
      if !call.is_null() && does_call_error(&*call) {
        ControlFlow::Throws
      } else {
        ControlFlow::None
      }
    }
  }
}
