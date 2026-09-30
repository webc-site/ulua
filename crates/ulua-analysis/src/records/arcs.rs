//! A filtered copy of a node's adjacency, used by `drain` to reason about the
//! subgraph induced by the pending queue. Mirrors C++ `Luau::detail::Arcs`
//! (`Analysis/src/TopoSortStatements.cpp:72`), with arena indices replacing the
//! `std::set<Node*>`.

use alloc::collections::BTreeSet;

use crate::records::node::NodeId;

#[derive(Debug, Clone, Default)]
pub struct Arcs {
  pub(crate) provides: BTreeSet<NodeId>,
  pub(crate) depends: BTreeSet<NodeId>,
}

impl Arcs {
  pub fn new() -> Self {
    Self {
      provides: BTreeSet::new(),
      depends: BTreeSet::new(),
    }
  }
}
