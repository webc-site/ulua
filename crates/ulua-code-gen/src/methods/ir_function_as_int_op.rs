use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn as_int_op(&mut self, op: IrOp) -> Option<i32> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    match self.const_op(op) {
      IrConst::Int(value) => Some(value),
      _ => None,
    }
  }
}
