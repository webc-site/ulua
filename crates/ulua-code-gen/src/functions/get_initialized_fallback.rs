use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

pub fn get_initialized_fallback(build: &mut IrBuilder, fallback: &mut IrOp, pcpos: i32) -> IrOp {
  if fallback.kind() == IrOpKind::None {
    *fallback = build.fallback_block(pcpos as u32);
  }

  *fallback
}
