use core::cmp::max;

use crate::{
  records::{
    bc_op::BcOp, block_producers::BlockProducers, bytecode_graph_parser::BytecodeGraphParser,
  },
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphParser<'a> {
  pub fn add_producer(&mut self, reg: Reg, op: BcOp) {
    let block_producers: &mut BlockProducers =
      &mut self.producers[self.current_block.index as usize];

    block_producers.own.insert(reg, op);

    self.func.regs.insert(op, reg);

    block_producers.invalid_after = max(reg as i32, block_producers.invalid_after);
  }
}
