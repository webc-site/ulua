use ulua_ast::records::ast_stat_repeat::AstStatRepeat;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  functions::matches::matches,
  records::{data_flow_graph_builder::DataFlowGraphBuilder, push_scope::PushScope},
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `r` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_repeat(&mut self, r: *mut AstStatRepeat) -> ControlFlow {
    let repeat_scope = self.make_child_scope(ScopeType::Loop);

    let cf = {
      let _ps = PushScope::new(&mut self.scope_stack, repeat_scope);
      let cf = unsafe { self.visit_block_without_child_scope((*r).body) };
      let _ = self.visit_expr_ast_expr(unsafe { (*r).condition });
      cf
    };

    unsafe {
      (*self.current_scope()).inherit(repeat_scope);
    }

    if matches(cf, ControlFlow::Breaks) || matches(cf, ControlFlow::Continues) {
      ControlFlow::None
    } else {
      cf
    }
  }
}
