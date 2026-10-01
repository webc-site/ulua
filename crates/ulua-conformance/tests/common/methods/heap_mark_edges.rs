use crate::common::records::heap::Heap;

impl Heap {
  /// cpp `Heap::markEdges()`（`tests/Conformance.test.cpp:3443-3465`）：从 `root` 出发把
  /// 所有可达节点标记为 marked，并返回这些节点的 `size` 之和（每个节点只计一次）。
  ///
  /// 上游是递归 DFS；这里用显式栈，语义等价（递归版对已 marked 的节点也只会继续展开
  /// 尚未 marked 的孩子，跳过已 marked 节点不会漏掉任何可达节点），且不受引用链深度
  /// 限制——移植的递归在测试进程的默认栈上反而更容易先爆栈。
  pub fn mark_edges(&mut self, root: usize) -> usize {
    let mut total = 0usize;
    let mut stack = vec![root];

    while let Some(ptr) = stack.pop() {
      let node = self
        .nodes
        .get_mut(&ptr)
        .expect("GCDump: link() 已经保证每条边的目标都在节点表里");

      if node.marked {
        continue;
      }

      node.marked = true;
      total += node.size;

      if let Some(children) = self.children.get(&ptr) {
        stack.extend(children.iter().copied());
      }
    }

    total
  }
}
