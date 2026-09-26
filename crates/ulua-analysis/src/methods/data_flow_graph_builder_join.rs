use crate::records::{data_flow_graph_builder::DataFlowGraphBuilder, dfg_scope::DfgScope};

impl DataFlowGraphBuilder {
  /// # Safety
  /// `p、a、b` 须指向 `DataFlowGraphBuilder` 在本次建图期间持有的存活 `DfgScope`：非空、对齐，地址
  /// 随 bump 分配稳定不移动；本函数从 a/b 读取、写入 p，调用方单线程独占，函数返回后指针仍由 builder 持有。
  /// 对应 C++ `void DataFlowGraphBuilder::join(DfgScope* p, DfgScope* a, DfgScope* b)` (`cpp/Analysis/src/DataFlowGraph.cpp:221`)。
  pub unsafe fn join(&mut self, p: *mut DfgScope, a: *mut DfgScope, b: *mut DfgScope) {
    // Safety: p/a/b 是 DFG 构建遍历传入的指向 self.scopes（PinnedStorage，仅追加、地址不
    // 移动）的存活非空 DfgScope 节点，`&*a`/`&*b` 重建的共享借用覆盖本次调用；join_bindings
    // 的 pub-unsafe 契约显式容忍 p 与 a 别名（先快照 a/b.bindings 再写 p.bindings），故调用
    // 期间不会同时持有对同一 scope bindings 的 & 与 &mut，单线程顺序构建亦无并发访问。
    unsafe { self.join_bindings(p, &*a, &*b) };
    // Safety: 与上一行同前提——a/b 指向 pinned arena 存活节点，join_props 契约同样先快照
    // 再改写，避免 p 别名 a 时形成 &/&mut 重叠，借用有效期仅覆盖该次调用。
    unsafe { self.join_props(p, &*a, &*b) };
  }
}
