use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_block::BcBlock, bc_function::BcFunction, bc_op::BcOp},
};

impl BcFunction {
  pub fn add_block(&mut self) -> BcOp {
    self.blocks.push(BcBlock::default());
    BcOp::bc_op_bc_op_kind_u32(BcOpKind::Block, (self.blocks.len() - 1) as u32)
  }
}
