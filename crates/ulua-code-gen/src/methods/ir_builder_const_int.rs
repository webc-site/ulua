use crate::{
  records::{ir_builder::IrBuilder, ir_const::IrConst, ir_op::IrOp},
};

impl IrBuilder {
  pub fn const_int(&mut self, value: i32) -> IrOp {
    self.const_any(IrConst::Int(value), value as u64)
  }
}
