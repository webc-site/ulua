use crate::{
  records::{
    bc_op::BcOp, block_producers::BlockProducers, bytecode_graph_parser::BytecodeGraphParser,
  },
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphParser<'a> {
  pub fn apply_call(
    &mut self,
    producers: &mut BlockProducers,
    call_op: BcOp,
    target_reg: Reg,
    nresults: i32,
  ) {
    producers.own.retain(|&reg, _| reg < target_reg);
    producers.cached.retain(|&reg, _| reg < target_reg);

    if nresults < 0 {
      producers.multi_return = call_op;
      producers.multi_return_start = target_reg;
      producers.invalid_after = 255;
    } else {
      producers.invalid_after = (target_reg as i32) - 1 + nresults;
    }
  }
}
