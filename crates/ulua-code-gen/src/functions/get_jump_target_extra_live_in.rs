use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{
    is_non_terminating_jump::is_non_terminating_jump,
    require_variadic_sequence::require_variadic_sequence,
  },
  macros::{codegen_assert::CODEGEN_ASSERT, op_a::op_a},
  records::{
    ir_block::IrBlock, ir_inst::IrInst, ir_to_string_context::IrToStringContext,
    register_set::RegisterSet,
  },
};

pub fn get_jump_target_extra_live_in(
  ctx: &mut IrToStringContext,
  _block: &IrBlock,
  block_idx: u32,
  inst: &mut IrInst,
) -> RegisterSet {
  let mut extra_rs = RegisterSet::default();

  if block_idx as usize >= ctx.cfg.r#in.len() {
    return extra_rs;
  }

  let def_rs = ctx.cfg.r#in[block_idx as usize];

  // Find first block argument, for guard instructions (isNonTerminatingJump), that's the first and only one
  CODEGEN_ASSERT!(is_non_terminating_jump(inst.cmd));
  let mut op = op_a(inst);

  if let Some(block_op) = inst
    .ops
    .as_slice()
    .get(1..)
    .and_then(|ops| ops.iter().copied().find(|op| op.kind() == IrOpKind::Block))
  {
    op = block_op;
  }

  if op.kind() == IrOpKind::Block && (op.index() as usize) < ctx.cfg.r#in.len() {
    let in_rs = ctx.cfg.r#in[op.index() as usize];

    for ((extra, in_reg), def_reg) in extra_rs.regs.iter_mut().zip(&in_rs.regs).zip(&def_rs.regs) {
      *extra = in_reg & !def_reg;
    }

    if in_rs.vararg_seq {
      require_variadic_sequence(&mut extra_rs, &def_rs, in_rs.vararg_start);
    }
  }

  extra_rs
}
