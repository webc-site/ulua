use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::control_flow::ControlFlow, records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `b` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_block_without_child_scope(&mut self, b: *mut AstStatBlock) -> ControlFlow {
    LUAU_ASSERT!(!b.is_null());

    let mut first_control_flow: Option<ControlFlow> = None;

    unsafe {
      let b = &*b;
      let mut i = 0usize;
      while i < b.body.size {
        let stat = *b.body.data.add(i);
        let cf = self.visit_ast_stat(stat);
        if cf != ControlFlow::None && first_control_flow.is_none() {
          first_control_flow = Some(cf);
        }
        i += 1;
      }
    }

    first_control_flow.unwrap_or(ControlFlow::None)
  }
}
