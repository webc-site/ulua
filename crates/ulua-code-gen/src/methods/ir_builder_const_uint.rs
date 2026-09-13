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
  pub fn const_uint(&mut self, value: u32) -> IrOp {
    let constant = IrConst {
      kind: IrConstKind::Uint,
      value: unsafe { transmute::<u64, IrConstValue>(value as u64) },
    };
    self.const_any(constant, value as u64)
  }
}
