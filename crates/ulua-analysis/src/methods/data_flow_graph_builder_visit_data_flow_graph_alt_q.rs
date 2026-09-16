use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  records::{data_flow_graph_builder::DataFlowGraphBuilder, push_scope::PushScope},
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_type_alias(&mut self, t: *mut AstStatTypeAlias) -> ControlFlow {
    unsafe {
      let t = &*t;

      let unreachable = self.make_child_scope(ScopeType::Linear);
      let mut ps = PushScope::new(&mut self.scope_stack, unreachable);

      self.visit_generics(t.generics);
      self.visit_generic_packs(t.generic_packs);
      self.visit_type_ast_type(t.type_ptr);

      ps.pop();
    }

    ControlFlow::None
  }
}
