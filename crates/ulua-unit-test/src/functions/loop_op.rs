use ulua_bytecode::{
  enums::bc_block_edge_kind::BcBlockEdgeKind, records::bc_op::BcOp, type_aliases::bc_edges::BcEdges,
};

use crate::functions::get_block_op::get_block_op;

pub fn loop_op(edges: &BcEdges) -> BcOp {
  get_block_op(edges, BcBlockEdgeKind::Loop)
}
