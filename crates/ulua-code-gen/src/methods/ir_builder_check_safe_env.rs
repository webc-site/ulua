use crate::{
  enums::ir_cmd::IrCmd,
  records::{
    ir_block::{IrBlock, K_BLOCK_NO_START_PC},
    ir_builder::IrBuilder,
  },
};

impl IrBuilder {
  pub fn check_safe_env(&mut self, pcpos: i32) {
    let active_block_idx = self.active_block_idx as usize;
    let active: &mut IrBlock = &mut self.function.blocks[active_block_idx];

    const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;
    const K_BLOCK_FLAG_SAFE_ENV_CLEAR: u8 = 1 << 1;

    if active.startpc != K_BLOCK_NO_START_PC && (active.flags & K_BLOCK_FLAG_SAFE_ENV_CLEAR) == 0 {
      active.flags |= K_BLOCK_FLAG_SAFE_ENV_CHECK;
    }

    let exit_op = self.vm_exit(pcpos as u32);
    self.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, exit_op);
  }
}
