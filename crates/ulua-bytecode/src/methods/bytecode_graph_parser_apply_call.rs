use crate::{
  records::{
    bc_op::BcOp, block_producers::BlockProducers, bytecode_graph_parser::BytecodeGraphParser,
  },
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `applyCall(BlockProducers&, BcOp, Reg, int)`（`BytecodeGraphParser.h:366-396`）。
  ///
  /// 不读取解析器自身状态，故为无接收者的关联函数：调用方可直接
  /// `&mut self.producers[..]`，不必再用裸指针绕开借用检查。
  pub fn apply_call(producers: &mut BlockProducers, call_op: BcOp, target_reg: Reg, nresults: i32) {
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
