use crate::{
  enums::condition_x_64::ConditionX64,
  records::{ir_block::IrBlock, ir_lowering_x_64::IrLoweringX64, ir_op::IrOp, label::Label},
};

impl IrLoweringX64 {
  pub fn jump_or_abort_on_undef_condition_x_64_ir_op_u32_ir_block(
    &mut self,
    cond: ConditionX64,
    target: IrOp,
    index: u32,
    next: &IrBlock,
  ) {
    let mut fresh = Label { id: 0, location: 0 };
    self.jump_or_abort_on_undef_no_finalize(cond, target, index, next, &mut fresh);
    self.finalize_target_label(target, index, &mut fresh);
  }
}
