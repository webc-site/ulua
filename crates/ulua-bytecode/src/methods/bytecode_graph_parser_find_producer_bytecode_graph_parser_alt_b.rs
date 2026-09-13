use std::collections::HashSet;

use crate::{
  records::{bc_op::BcOp, bc_op_hash::BcOpHash, bytecode_graph_parser::BytecodeGraphParser},
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphParser<'a> {
  pub fn find_producer_bc_op_reg(&mut self, block: BcOp, reg: Reg) -> Option<BcOp> {
    let mut visited: HashSet<BcOp, BcOpHash> = HashSet::default();
    self.find_producer_bc_op_reg_unordered_set_bc_op_bc_op_hash(block, reg, &mut visited)
  }
}
