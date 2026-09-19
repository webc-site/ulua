use std::ptr::null_mut;

use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

impl IrFunction {
  /// 对应 cpp `IrFunction::asInstOp`；越界索引返回空指针而非未定义行为。
  pub fn as_inst_op(&mut self, op: IrOp) -> *mut IrInst {
    if op.kind() == IrOpKind::Inst {
      self
        .instructions
        .get_mut(op.index() as usize)
        .map_or(null_mut(), |inst| inst as *mut IrInst)
    } else {
      null_mut()
    }
  }

  /// 只读版：非 Inst kind 或越界返回 `None`。用于纯读路径，避免强制调用方持有
  /// `&mut IrFunction`（try_get_operand_tag 等）。
  pub fn as_inst_op_ref(&self, op: IrOp) -> Option<&IrInst> {
    if op.kind() == IrOpKind::Inst {
      self.instructions.get(op.index() as usize)
    } else {
      None
    }
  }
}
