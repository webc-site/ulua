//! Populates the collector's declaration map and wires `IdentifierHash` into
//! the container's `DenseHasher` trait. Mirrors the body of the C++
//! `ArcCollector::ArcCollector` loop (`Analysis/src/TopoSortStatements.cpp:207-217`):
//! for every node with a name, register the *first* node declaring that name.
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{arc_collector::ArcCollector, node::NodeId};

impl ArcCollector<'_> {
  /// Build `map: name → first declaring node` over `nodes` (indices into the
  /// arena). `default()` 起步的空键占位为 `Identifier::default()`（`("", null)`，
  /// 与 cpp `map{}` 起步同构）；占用由位图判定，该占位不是保留键（见
  /// ulua-common `dense_hash_table` 模块文档）。
  pub fn populate_map(&mut self, nodes: impl Iterator<Item = NodeId>) {
    self.map = DenseHashMap::default();

    for index in nodes {
      let name = match &self.arena[index].name {
        Some(name) => name.clone(),
        None => continue,
      };
      if !self.map.contains(&name) {
        *self.map.get_or_insert(name) = index;
      }
    }
  }
}
