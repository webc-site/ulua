use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::predecessors::predecessors,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_lowering_x_64::IrLoweringX64},
};

impl IrLoweringX64 {
  pub fn finish_block(&mut self, curr: &IrBlock, next: &IrBlock) {
    if !self.regs.spills.is_empty() {
      let function = unsafe { &*self.function };
      let next_idx = function.get_block_index(next);
      let curr_idx = function.get_block_index(curr);

      for pred_idx in predecessors(&function.cfg, next_idx) {
        let pred_block = &function.blocks[pred_idx as usize];
        CODEGEN_ASSERT!(pred_idx == curr_idx || pred_block.kind == IrBlockKind::Dead);
      }

      CODEGEN_ASSERT!(next.use_count == 1);
    }
  }
}
