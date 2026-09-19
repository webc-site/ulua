use crate::{
  records::{ir_builder::IrBuilder, ir_const::IrConst, ir_op::IrOp},
};

impl IrBuilder {
  pub fn const_int_64(&mut self, value: i64) -> IrOp {
    self.const_any(IrConst::Int64(value), value as u64)
  }
}
