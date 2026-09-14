//! Faithful port of `Luau::detail::prune`
//! (`Analysis/src/TopoSortStatements.cpp:386-401`).
//!
//! ```cpp
//! // Clip arcs to and from the node
//! void prune(Node* next)
//! {
//!     for (const auto& node : next->provides)
//!     {
//!         auto it = node->depends.find(next);
//!         LUAU_ASSERT(it != node->depends.end());
//!         node->depends.erase(it);
//!     }
//!
//!     for (const auto& node : next->depends)
//!     {
//!         auto it = node->provides.find(next);
//!         LUAU_ASSERT(it != node->provides.end());
//!         node->provides.erase(it);
//!     }
//! }
//! ```
/// Clip arcs to and from the node.
///
use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::node::Node;
/// # Safety
/// `next` and every pointer reachable through its `provides` / `depends` sets
/// must be live `Node`s. None of the visited nodes alias `next` (the graph has
/// no self-edges), so erasing `next` from their sets is sound.
pub unsafe fn prune(next: *mut Node) {
  // The C++ loops iterate `next->provides` / `next->depends` (which are never
  // mutated here) while erasing `next` from *other* nodes' sets. Snapshot the
  // pointer sets first so the mutation of the neighbours cannot alias the
  // borrow of `next`.
  let provides: Vec<*mut Node> = unsafe { (*next).provides.iter().copied().collect() };
  let depends: Vec<*mut Node> = unsafe { (*next).depends.iter().copied().collect() };

  for node in provides {
    let removed = unsafe { (*node).depends.remove(&next) };
    LUAU_ASSERT!(removed);
  }

  for node in depends {
    let removed = unsafe { (*node).provides.remove(&next) };
    LUAU_ASSERT!(removed);
  }
}
