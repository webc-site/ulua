use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_op::LUAU_INSN_OP},
};

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind},
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_for_n_loop(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;

  let repeat_jump_target = get_jump_target(unsafe { *pc }, pcpos as u32);
  let loop_repeat = build.block_at_inst(repeat_jump_target as u32);
  let op = LuauOpcode::from(LUAU_INSN_OP(unsafe { *pc }) as u8);
  let op_length = get_op_length(op);
  let loop_exit = build.block_at_inst((pcpos + op_length) as u32);

  CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
  let loop_info = build.numeric_loop_stack.last().copied().unwrap();
  let step_k = loop_info.step;

  if repeat_jump_target != loop_info.startpc {
    let pcpos_op = build.const_uint(pcpos as u32);
    build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos_op);
  }

  let reg_limit = build.vm_reg(ra);
  let limit = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_limit);
  let step = if step_k.kind() == IrOpKind::Undef {
    let reg_step = build.vm_reg(ra + 1);
    build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_step)
  } else {
    step_k
  };

  let reg_idx = build.vm_reg(ra + 2);
  let mut idx = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_idx);
  idx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::AddNum, idx, step);
  let reg_idx = build.vm_reg(ra + 2);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_idx, idx);

  if step_k.kind() == IrOpKind::Undef {
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::JumpFornLoopCond,
      idx,
      limit,
      step,
      loop_repeat,
      loop_exit,
    );
  } else {
    let step_n = build.function.double_op(step_k);

    let reg_step = build.vm_reg(ra + 1);
    let one = build.const_int(1);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::MarkUsed, reg_step, one);

    // 原译两个分支同为 LessEqual,直接取值
    let cond = IrCondition::LessEqual;
    let cond_op = build.cond(cond);

    if step_n > 0.0 {
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpNum,
        idx,
        limit,
        cond_op,
        loop_repeat,
        loop_exit,
      );
    } else {
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpNum,
        limit,
        idx,
        cond_op,
        loop_repeat,
        loop_exit,
      );
    }
  }

  if build.is_internal_block(loop_exit) {
    build.begin_block(loop_exit);
  }
}
