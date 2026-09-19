use crate::{
  records::{ir_builder::IrBuilder, ir_const::IrConst, ir_op::IrOp},
};

impl IrBuilder {
  pub fn const_uint(&mut self, value: u32) -> IrOp {
    self.const_any(IrConst::Uint(value), value as u64)
  }
}
