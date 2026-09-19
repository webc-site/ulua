use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{bc_function::BcFunction, bc_op::BcOp, block_producers::BlockProducers};

#[derive(Debug)]
pub struct BytecodeGraphParser<'a> {
  pub(crate) func: &'a mut BcFunction,
  pub(crate) block_by_pc: DenseHashMap<u32, BcOp>,
  pub(crate) producers: Vec<BlockProducers>,
  pub(crate) current_block: BcOp,
}

impl<'a> BytecodeGraphParser<'a> {
  pub(crate) const K_MAX_CFG_BLOCKS: u32 = 1000;
}
