use crate::common::records::heap::Heap;

impl Heap {
  /// cpp `Heap::findNodeByPtr()`（`tests/Conformance.test.cpp:3432-3441`）：上游线性比较
  /// `v.ptr == ptr`；本端 `nodes` 就以该指针为键，故一次哈希查找即可，返回的仍是同一个
  /// 指针键（见 `find_node_by_name` 的说明）。
  pub fn find_node_by_ptr(&self, ptr: usize) -> Option<usize> {
    self.nodes.contains_key(&ptr).then_some(ptr)
  }
}
