use ulua_bytecode::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::bc_block::BcBlock,
  type_aliases::{bc_edges::BcEdges, comp_time_bc_function::CompTimeBcFunction},
};

use crate::functions::get_block::get_block;

pub fn fallthrough_block<'a>(
  fn_: &'a mut CompTimeBcFunction,
  edges: &'a mut BcEdges,
) -> &'a mut BcBlock {
  get_block(fn_, edges, BcBlockEdgeKind::Fallthrough)
}
