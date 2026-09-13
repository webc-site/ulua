use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn update_last_use_locations_in_block(function: &mut IrFunction, block_idx: u32) {
  let block = &function.blocks[block_idx as usize];
  let start = block.start;
  let finish = block.finish;

  for inst_idx in start..=finish {
    let ops = function.instructions[inst_idx as usize].ops.clone();

    for op in ops.iter() {
      let op: IrOp = *op;
      if op.kind() == IrOpKind::Inst {
        function.instructions[op.index() as usize].last_use = inst_idx;
      }
    }
  }
}
