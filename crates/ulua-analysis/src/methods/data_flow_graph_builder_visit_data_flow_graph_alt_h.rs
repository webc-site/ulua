use ulua_ast::records::ast_stat_return::AstStatReturn;

use crate::{
  enums::control_flow::ControlFlow, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `r` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_return(&mut self, r: *mut AstStatReturn) -> ControlFlow {
    unsafe {
      let list = (*r).list;
      for &e in list.as_slice() {
        let _ = self.visit_expr_ast_expr(e);
      }
    }

    ControlFlow::Returns
  }
}
