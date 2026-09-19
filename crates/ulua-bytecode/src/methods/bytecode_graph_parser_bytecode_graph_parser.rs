use std::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::{
  bc_function::BcFunction, bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser,
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `BytecodeGraphParser::BytecodeGraphParser(BcFunction& fn)`
  pub fn new(func: &'a mut BcFunction) -> Self {
    Self {
      func,
      block_by_pc: DenseHashMap::new(u32::MAX - 1),
      producers: Vec::new(),
      current_block: BcOp::new(),
    }
  }
}
