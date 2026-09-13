//! Faithful port of `Luau::detail::ArcCollector::ArcCollector`
//! (`Analysis/src/TopoSortStatements.cpp:207-217`).
//!
//! ```cpp
//! ArcCollector(NodeQueue& queue)
//!     : queue(queue)
//!     , map(Identifier{std::string{}, 0})
//!     , currentArc(nullptr)
//! {
//!     for (const auto& node : queue)
//!     {
//!         if (node->name && !map.contains(*node->name))
//!             map[*node->name] = node.get();
//!     }
//! }
//! ```
// Wires the C++ `IdentifierHash::operator()` functor (used as the `Hash`
// template parameter of `DenseHashMap<Identifier, Node*, IdentifierHash>`) into
// the container's `DenseHasher` trait so the map is constructible. The hash body
// itself already lives in `IdentifierHash::identifier_hash_operator_call`.
use alloc::string::String;
use core::ptr::{null, null_mut};

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseHasher};

use crate::{
  records::{
    arc_collector::ArcCollector, identifier::Identifier, identifier_hash::IdentifierHash,
    node::Node,
  },
  type_aliases::node_queue::NodeQueue,
};
impl DenseHasher<Identifier> for IdentifierHash {
  fn hash(&self, key: &Identifier) -> usize {
    IdentifierHash::identifier_hash_operator_call(key)
  }
}

impl ArcCollector {
  pub fn arc_collector(&mut self, queue: &mut NodeQueue) {
    // : queue(queue), map(Identifier{std::string{}, 0}), currentArc(nullptr)
    self.queue = queue as *mut NodeQueue;
    self.map = DenseHashMap::new(Identifier::new(String::new(), null()));
    self.current_arc = null_mut();

    // for (const auto& node : queue)
    //     if (node->name && !map.contains(*node->name))
    //         map[*node->name] = node.get();
    for &node_ptr in queue.iter() {
      let node = unsafe { &*node_ptr };
      if let Some(name) = &node.name
        && !self.map.contains(name)
      {
        *self.map.get_or_insert(name.clone()) = node_ptr;
      }
    }
  }
}
