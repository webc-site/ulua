use crate::{
  enums::{ir_const_kind::IrConstKind, ir_op_kind::IrOpKind},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  pub fn as_int_64_op(&mut self, op: IrOp) -> Option<i64> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    let value = self.const_op(op);

    if value.kind != IrConstKind::Int64 {
      return None;
    }

    Some(unsafe { value.value.value_int64 })
  }
}
