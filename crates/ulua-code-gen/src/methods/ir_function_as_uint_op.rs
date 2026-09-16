use crate::{
  enums::{ir_const_kind::IrConstKind, ir_op_kind::IrOpKind},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  /// cpp IrData.h:1615 `asUintOp`；上游同族 API，本成员暂无调用点，保留 API 表面。
  pub fn as_uint_op(&mut self, op: IrOp) -> Option<u32> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    let value = self.const_op(op);

    if value.kind != IrConstKind::Uint {
      return None;
    }

    unsafe { Some(value.value.value_uint) }
  }
}
