use crate::records::{ir_builder::IrBuilder, ir_const::IrConst, ir_op::IrOp};

impl IrBuilder {
  pub fn const_tag(&mut self, value: u8) -> IrOp {
    self.const_any(IrConst::Tag(value), value as u64)
  }
}
