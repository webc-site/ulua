use crate::records::{data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope};

impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `p、`a、`b` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn join(&mut self, p: *mut DfgScope, a: *mut DfgScope, b: *mut DfgScope) {
    unsafe { self.join_bindings(p, &*a, &*b) };
    unsafe { self.join_props(p, &*a, &*b) };
  }
}
