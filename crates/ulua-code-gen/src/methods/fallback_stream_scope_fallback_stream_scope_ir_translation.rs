use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder, ir_op::IrOp},
};

impl<'a> FallbackStreamScope<'a> {
  pub fn fallback_stream_scope(&mut self, build: &mut IrBuilder, fallback: IrOp, next: IrOp) {
    CODEGEN_ASSERT!(fallback.kind() == IrOpKind::Block);
    CODEGEN_ASSERT!(next.kind() == IrOpKind::Block);

    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
    build.begin_block(fallback);
  }
}
