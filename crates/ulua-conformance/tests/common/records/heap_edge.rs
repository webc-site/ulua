/// cpp `tests/Conformance.test.cpp:3392-3398` 的 `struct HeapEdge`。
///
/// 上游还有一个 `HeapNode* node` 缓存，由 `Heap::link()` 解析出来；Rust 的借用模型下
/// 节点表与边表不能互相引用，故该缓存改由 `Heap::children` 邻接表承担。
#[derive(Debug)]
pub struct HeapEdge {
  pub source: usize,
  pub target: usize,
  /// 上游 `std::string name`（`nullptr` 时是空串）；用 `Option` 区分「没有名字」与「名字是空串」。
  pub name: Option<String>,
}
