use crate::{
  enums::ir_const_kind::IrConstKind,
  records::{
    ir_builder::IrBuilder,
    ir_const::{IrConst, IrConstValue},
    ir_op::IrOp,
  },
};

impl IrBuilder {
  pub fn const_int_64(&mut self, value: i64) -> IrOp {
    let constant = IrConst {
      kind: IrConstKind::Int64,
      value: IrConstValue { value_int64: value },
    };
    self.const_any(constant, value as u64)
  }
}
