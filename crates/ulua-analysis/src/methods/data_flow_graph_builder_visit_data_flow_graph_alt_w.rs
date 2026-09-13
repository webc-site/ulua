use ulua_ast::records::ast_stat_error::AstStatError;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  records::{
    data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope, push_scope::PushScope,
  },
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `error` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_error(&mut self, error: *mut AstStatError) -> ControlFlow {
    unsafe {
      let error = &*error;

      let unreachable: *mut DfgScope = self.make_child_scope(ScopeType::Linear);
      let mut ps = PushScope::new(&mut self.scope_stack, unreachable);

      let mut i = 0usize;
      while i < error.statements.size {
        let s = *error.statements.data.add(i);
        self.visit_ast_stat(s);
        i += 1;
      }

      let mut i = 0usize;
      while i < error.expressions.size {
        let e = *error.expressions.data.add(i);
        self.visit_expr_ast_expr(e);
        i += 1;
      }

      ps.pop();
    }

    ControlFlow::None
  }
}
