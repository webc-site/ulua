use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

pub fn update_use_counts(function: &mut IrFunction) {
  for block in &mut function.blocks {
    block.use_count = 0;
  }

  for inst in &mut function.instructions {
    inst.use_count = 0;
  }

  for inst_idx in 0..function.instructions.len() {
    let ops = function.instructions[inst_idx].ops.clone();

    for op in ops.iter() {
      let op: IrOp = *op;
      match op.kind() {
        IrOpKind::Inst => {
          let target: &mut IrInst = &mut function.instructions[op.index() as usize];
          debug_assert!(target.use_count < 0xffff);
          target.use_count += 1;
        }
        IrOpKind::Block => {
          let target = &mut function.blocks[op.index() as usize];
          debug_assert!(target.use_count < 0xffff);
          target.use_count += 1;
        }
        _ => {}
      }
    }
  }
}
