use ulua_common::FFlag;

use crate::{
  enums::{condition_x_64::ConditionX64, ir_op_kind::IrOpKind},
  functions::get_negated_condition_condition_x_64::get_negated_condition,
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
    if FFlag::LuauCodegenVmExitSync.get() {
      let mut fresh = Label { id: 0, location: 0 };
      self.jump_or_abort_on_undef_no_finalize(cond, target, index, next, &mut fresh);
      self.finalize_target_label(target, index, &mut fresh);
    } else {
      let mut fresh = Label { id: 0, location: 0 };
      let label = self.get_target_label(target, index, &mut fresh) as *mut Label;

      unsafe {
        if target.kind() == IrOpKind::Undef {
          if cond == ConditionX64::Count {
            (*self.build).ud_2();
          } else {
            (*self.build).jcc(get_negated_condition(cond), &mut *label);
            (*self.build).ud_2();
            (*self.build).set_label(&mut *label);
          }
        } else if cond == ConditionX64::Count {
          let should_jump = if target.kind() == IrOpKind::VmExit {
            true
          } else {
            let target_block = self.block_op(target);
            !self.is_fallthrough_block(&*target_block, next)
          };

          if should_jump {
            (*self.build).jmp_label(&mut *label);
          }
        } else {
          (*self.build).jcc(cond, &mut *label);
        }
      }

      self.finalize_target_label(target, index, &mut fresh);
    }
  }
}
