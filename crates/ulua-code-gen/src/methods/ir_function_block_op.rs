use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_block::IrBlock, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn block_op(&mut self, op: IrOp) -> &mut IrBlock {
    assert!(op.kind() == IrOpKind::Block);
    &mut self.blocks[op.index() as usize]
  }
}
