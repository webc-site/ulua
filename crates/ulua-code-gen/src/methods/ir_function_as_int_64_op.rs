use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn as_int_64_op(&mut self, op: IrOp) -> Option<i64> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    match self.const_op(op) {
      IrConst::Int64(value) => Some(value),
      _ => None,
    }
  }
}
