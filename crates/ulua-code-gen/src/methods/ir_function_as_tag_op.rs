use crate::{
  enums::{ir_const_kind::IrConstKind, ir_op_kind::IrOpKind},
  records::{ir_const::IrConst, ir_function::IrFunction, ir_op::IrOp},
};

impl IrFunction {
  /// cpp IrData.h:1544 `asTagOp`；上游同族 API（asIntOp/asDoubleOp 在用），本成员暂无调用点，保留 API 表面。
  pub fn as_tag_op(&mut self, op: IrOp) -> Option<u8> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    let value: IrConst = IrFunction::const_op(self, op);

    if value.kind != IrConstKind::Tag {
      return None;
    }

    unsafe { Some(value.value.value_tag) }
  }
}
