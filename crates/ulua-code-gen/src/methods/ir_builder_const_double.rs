use crate::records::{ir_builder::IrBuilder, ir_const::IrConst, ir_op::IrOp};

impl IrBuilder {
  pub fn const_double(&mut self, value: f64) -> IrOp {
    // 去重键取位模式，规避 NaN 内容不等导致的重复常量
    self.const_any(IrConst::Double(value), value.to_bits())
  }
}
