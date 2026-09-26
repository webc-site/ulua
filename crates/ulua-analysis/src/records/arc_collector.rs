use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  identifier::Identifier,
  identifier_hash::IdentifierHash,
  node::{Node, NodeId},
};

/// Visitor that records the use→declaration dependency arcs over the toposort
/// arena. It holds the arena mutably and stores nodes in [`map`] /
/// [`current_arc`] by index, so no raw `Node*` escapes into the graph logic.
///
/// [`map`]: Self::map
/// [`current_arc`]: Self::current_arc
#[derive(Debug)]
pub struct ArcCollector<'arena> {
  pub arena: &'arena mut Vec<Node>,
  pub map: DenseHashMap<Identifier, NodeId, IdentifierHash>,
  /// The node currently being visited. `None` until `toposort` positions the
  /// cursor; `add` is only ever invoked while it is `Some`.
  pub current_arc: Option<NodeId>,
}
