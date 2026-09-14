use ulua_common::FFlag;

use crate::{
  enums::{code_gen_counter::CodeGenCounter, ir_block_kind::IrBlockKind},
  records::{
    ir_block::{IrBlock, K_BLOCK_NO_START_PC},
    ir_lowering_x_64::IrLoweringX64,
  },
};

impl IrLoweringX64 {
  pub fn start_block(&mut self, curr: &IrBlock) {
    if curr.startpc != K_BLOCK_NO_START_PC {
      let counter = if curr.kind == IrBlockKind::Fallback {
        CodeGenCounter::FallbackBlockExecuted
      } else {
        CodeGenCounter::RegularBlockExecuted
      };
      self.ir_lowering_x_64_alloc_and_increment_counter_at(counter, curr.startpc);
    }

    if FFlag::LuauCodegenVmExitSync.get() && curr.kind == IrBlockKind::ExitSync {
      let block_index = unsafe { (*self.function).get_block_index(curr) };
      self.regs.setup_exit_sync_entry(block_index);
    }
  }
}
