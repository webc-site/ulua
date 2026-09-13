use ulua_bytecode::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::bc_block::BcBlock,
  type_aliases::{bc_edges::BcEdges, comp_time_bc_function::CompTimeBcFunction},
};

use crate::functions::get_block_op::get_block_op;

pub fn get_block<'a>(
  fn_: &'a mut CompTimeBcFunction,
  edges: &'a mut BcEdges,
  kind: BcBlockEdgeKind,
) -> &'a mut BcBlock {
  let op = get_block_op(edges, kind);
  fn_.block_op(op)
}
