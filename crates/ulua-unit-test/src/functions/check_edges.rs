use ulua_bytecode::{enums::bc_block_edge_kind::BcBlockEdgeKind, type_aliases::bc_edges::BcEdges};

pub fn check_edges(edges: &BcEdges, expected_edges: &[BcBlockEdgeKind]) -> bool {
  if edges.len() != expected_edges.len() {
    return false;
  }

  for (edge, expected) in edges.iter().zip(expected_edges.iter()) {
    if edge.kind != *expected {
      return false;
    }
  }

  true
}
