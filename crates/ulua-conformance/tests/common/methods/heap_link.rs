use ulua_common::collections::HashMap;

use crate::common::records::heap::Heap;

impl Heap {
  /// cpp `Heap::link()`（`tests/Conformance.test.cpp:3405-3419`）：把边表挂成邻接表。
  ///
  /// 上游用两个 `REQUIRE` 保证边的两端都出现在节点表里（枚举器漏报节点即为回归），
  /// 这里等价为断言，并把边名带进失败信息便于定位。
  pub fn link(&mut self) {
    let mut children: HashMap<usize, Vec<usize>> = HashMap::default();

    for edge in &self.edges {
      assert!(
        self.nodes.contains_key(&edge.target),
        "GCDump: edge {:#x} -> {:#x} ({}) points at a node that was not enumerated",
        edge.source,
        edge.target,
        edge.name.as_deref().unwrap_or("<unnamed>"),
      );
      assert!(
        self.nodes.contains_key(&edge.source),
        "GCDump: edge {:#x} -> {:#x} ({}) originates from a node that was not enumerated",
        edge.source,
        edge.target,
        edge.name.as_deref().unwrap_or("<unnamed>"),
      );

      children.entry(edge.source).or_default().push(edge.target);
    }

    self.children = children;
  }
}
