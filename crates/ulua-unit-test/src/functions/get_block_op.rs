use ulua_bytecode::{
  enums::bc_block_edge_kind::BcBlockEdgeKind, records::bc_op::BcOp, type_aliases::bc_edges::BcEdges,
};

pub fn get_block_op(edges: &BcEdges, kind: BcBlockEdgeKind) -> BcOp {
  for edge in edges.iter() {
    if edge.kind == kind {
      return edge.target;
    }
  }

  panic!("missing bytecode block edge: {:?}", kind)
}
