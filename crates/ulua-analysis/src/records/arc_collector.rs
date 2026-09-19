use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{identifier::Identifier, identifier_hash::IdentifierHash, node::Node},
  type_aliases::node_queue::NodeQueue,
};

#[derive(Debug, Clone)]
pub struct ArcCollector {
  pub queue: *mut NodeQueue,
  pub map: DenseHashMap<Identifier, *mut Node, IdentifierHash>,
  pub current_arc: *mut Node,
}
