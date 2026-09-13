use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  records::{data_flow_graph_builder::DataFlowGraphBuilder, push_scope::PushScope},
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `b` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_block(&mut self, b: *mut AstStatBlock) -> ControlFlow {
    LUAU_ASSERT!(!b.is_null());

    let child = self.make_child_scope(ScopeType::Linear);

    let cf;
    {
      let ps = PushScope::new(&mut self.scope_stack, child);
      cf = unsafe { self.visit_block_without_child_scope(b) };
      drop(ps);
    }

    unsafe {
      (*self.current_scope()).inherit(child);
    }
    cf
  }
}
