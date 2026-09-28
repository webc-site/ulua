/// cpp `tests/Conformance.test.cpp:3379-3390` 的 `struct HeapNode`。
///
/// 与上游的两处结构性差异（都是为了适配 Rust 借用模型，语义等价）：
/// - 上游的 `std::vector<HeapEdge*> children` 改由 `Heap::children` 邻接表表达
///   （见 [`crate::common::records::heap::Heap`]）；
/// - 上游的 `memcat` 字段（`luaC_enumheap` 会报告对象所属的内存分类）在 GCDump 用例里
///   从未被读取，故不移植，避免只写不读的死字段。
#[derive(Debug)]
pub struct HeapNode {
  pub ptr: usize,
  pub tag: u8,
  pub size: usize,
  pub name: String,
  pub marked: bool,
}
