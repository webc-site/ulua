use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

#[derive(Debug)]
pub struct FallbackStreamScope<'a> {
  pub(crate) build: &'a mut IrBuilder,
  pub(crate) next: IrOp,
}

impl<'a> FallbackStreamScope<'a> {
  pub fn new(build: &'a mut IrBuilder, fallback: IrOp, next: IrOp) -> Self {
    ulua_common::LUAU_ASSERT!(fallback.kind() == IrOpKind::Block);
    ulua_common::LUAU_ASSERT!(next.kind() == IrOpKind::Block);

    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
    build.begin_block(fallback);

    Self { build, next }
  }
}

impl<'a> Drop for FallbackStreamScope<'a> {
  fn drop(&mut self) {
    self.build.begin_block(self.next);
  }
}
