use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::predecessors::predecessors,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_lowering_a_64::IrLoweringA64},
};

impl IrLoweringA64 {
  pub fn ir_lowering_a_64_finish_block(&mut self, curr: &IrBlock, next: &IrBlock) {
    if !self.regs.spills.is_empty() {
      let function = unsafe { &*self.function };
      let next_idx = function.get_block_index(next);
      let curr_idx = function.get_block_index(curr);

      let preds = predecessors(&function.cfg, next_idx);
      for pred_idx in preds {
        let pred_block = &function.blocks[pred_idx as usize];
        CODEGEN_ASSERT!(pred_idx == curr_idx || pred_block.kind == IrBlockKind::Dead);
      }

      CODEGEN_ASSERT!(next.use_count == 1);
    }
  }
}
