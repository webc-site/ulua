use crate::{
  enums::ir_block_kind::IrBlockKind,
  records::{ir_block::IrBlock, ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn is_internal_block(&self, block: IrOp) -> bool {
    let target: &IrBlock = &self.function.blocks[block.index() as usize];
    target.kind == IrBlockKind::Internal
  }
}
