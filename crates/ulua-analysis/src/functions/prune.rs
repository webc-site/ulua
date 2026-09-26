//! Clip the arcs to and from a node.
//!
//! Mirrors `Luau::detail::prune` (`Analysis/src/TopoSortStatements.cpp:386-401`).
//! The C++ iterates `next->provides`/`next->depends` (never mutated here) while
//! erasing `next` from each neighbour's opposite set; we snapshot the two
//! adjacency sets as owned index vectors first, so the neighbour mutations cannot
//! alias the source iteration — the same reasoning that required the pointer
//! version, now without `unsafe`.
use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::node::{Node, NodeId};

pub fn prune(arena: &mut [Node], next: NodeId) {
  let provides: Vec<NodeId> = arena[next].provides.iter().copied().collect();
  let depends: Vec<NodeId> = arena[next].depends.iter().copied().collect();

  for node in provides {
    let removed = arena[node].depends.remove(&next);
    LUAU_ASSERT!(removed);
  }

  for node in depends {
    let removed = arena[node].provides.remove(&next);
    LUAU_ASSERT!(removed);
  }
}
