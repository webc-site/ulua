use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  records::{data_flow_graph_builder::DataFlowGraphBuilder, push_scope::PushScope},
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_stat_type_function(
    &mut self,
    f: *mut AstStatTypeFunction,
  ) -> ControlFlow {
    unsafe {
      let unreachable = self.make_child_scope(ScopeType::Linear);
      let mut ps = PushScope::new(&mut self.scope_stack, unreachable);

      self.visit_expr_ast_expr_function((*f).body);

      ps.pop();
    }

    ControlFlow::None
  }
}
