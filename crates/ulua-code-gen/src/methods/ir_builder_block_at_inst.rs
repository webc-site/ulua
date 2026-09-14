use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_op_kind::IrOpKind},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn block_at_inst(&mut self, index: u32) -> IrOp {
    let block_index = self.inst_index_to_block[index as usize];
    if block_index != u32::MAX {
      return IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, block_index);
    }

    let result = self.block(IrBlockKind::Internal);
    self.function.block_op(result).startpc = index;
    result
  }
}
