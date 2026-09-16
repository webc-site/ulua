use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    can_invalidate_safe_env::can_invalidate_safe_env, is_block_terminator::is_block_terminator,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_builder::IrBuilder, ir_inst::IrInst, ir_op::IrOp, register_a_64::RegisterA64,
    register_x_64::RegisterX64,
  },
};

const K_BLOCK_FLAG_SAFE_ENV_CLEAR: u8 = 1 << 1;

impl IrBuilder {
  pub fn inst_ir_cmd_initializer_list_ir_op(&mut self, cmd: IrCmd, ops: &[IrOp]) -> IrOp {
    let index = self.function.instructions.len() as u32;
    self.function.instructions.push(IrInst {
      cmd,
      ops: ops.iter().cloned().collect(),
      last_use: 0,
      use_count: 0,
      reg_x64: RegisterX64::default(),
      reg_a64: RegisterA64::default(),
      reused_reg: false,
      spilled: false,
      needs_reload: false,
    });

    CODEGEN_ASSERT!(!self.in_terminated_block);

    if is_block_terminator(cmd) {
      self.function.blocks[self.active_block_idx as usize].finish = index;
      self.in_terminated_block = true;
    }

    if can_invalidate_safe_env(cmd) {
      self.function.blocks[self.active_block_idx as usize].flags |= K_BLOCK_FLAG_SAFE_ENV_CLEAR;
    }

    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, index)
  }
}
