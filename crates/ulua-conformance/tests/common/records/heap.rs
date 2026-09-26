use ulua_common::collections::HashMap;

use crate::common::records::{heap_edge::HeapEdge, heap_node::HeapNode};

/// cpp `tests/Conformance.test.cpp:3400-3419` 的 `struct Heap`：`luaC_enumheap` 回调
/// 收集到的堆快照（引用图）。
///
/// - `nodes` 以 GC 对象指针为键，对应上游 `Luau::DenseHashMap<void*, HeapNode>`；
/// - `edges` 是原始边表，同一源节点可以有多条出边，所以必须是 `Vec` 而不是以
///   `from` 为键的映射（否则每个源只剩最后一条边）；
/// - `children` 是 [`Heap::link`] 解析出的邻接表（源指针 -> 目标指针列表），
///   取代上游挂在 `HeapEdge*` 上的 `HeapNode::children`。
#[derive(Default)]
pub struct Heap {
  pub nodes: HashMap<usize, HeapNode>,
  pub edges: Vec<HeapEdge>,
  pub children: HashMap<usize, Vec<usize>>,
}
