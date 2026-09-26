use crate::common::records::heap::Heap;

impl Heap {
  /// cpp `Heap::findNodeByName()`（`tests/Conformance.test.cpp:3421-3430`）。
  ///
  /// 上游返回 `HeapNode*`；这里返回节点的指针键（它同时是 `nodes` 的键），既能继续
  /// 交给 `mark_edges`，也不会和后续 `&mut self` 借用冲突。
  pub fn find_node_by_name(&self, name: &str) -> Option<usize> {
    self
      .nodes
      .iter()
      .find(|(_, node)| node.name == name)
      .map(|(_, node)| node.ptr)
  }
}
