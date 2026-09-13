use ulua_ast::records::ast_stat_while::AstStatWhile;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  functions::matches::matches,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, push_scope::PushScope},
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `w` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_while(&mut self, w: *mut AstStatWhile) -> ControlFlow {
    let w = unsafe { &*w };
    let while_scope = self.make_child_scope(ScopeType::Loop);

    let cf = {
      let mut ps = PushScope::new(&mut self.scope_stack, while_scope);
      self.visit_expr_ast_expr(w.condition);
      let cf = self.visit_ast_stat_block(w.body);
      ps.pop();
      cf
    };

    let scope = self.current_scope();
    if !matches(cf, ControlFlow::Returns) && !matches(cf, ControlFlow::Throws) {
      unsafe { self.join(scope, scope, while_scope) };
    }

    ControlFlow::None
  }
}
