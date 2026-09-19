use core::mem::transmute;

use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{
    ir_builder::IrBuilder,
    ir_const::{IrConst, IrConstValue},
    ir_op::IrOp,
  },
};

impl IrBuilder {
  pub fn const_double(&mut self, value: f64) -> IrOp {
    let constant = IrConst {
      kind: IrConstKind::Double,
      value: unsafe { transmute::<u64, IrConstValue>(value.to_bits()) },
    };

    self.const_any(constant, value.to_bits())
  }
}
