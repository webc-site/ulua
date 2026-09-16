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
    // 逐值读取操作数（IrOp: Copy），避免克隆整条指令的操作数列表
    let op_count = function.instructions[inst_idx].ops.size() as usize;

    for k in 0..op_count {
      let op: IrOp = function.instructions[inst_idx].ops[k];
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
