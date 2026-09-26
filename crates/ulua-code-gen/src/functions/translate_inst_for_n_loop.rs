use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag::LuauBackedgeHeapCheck,
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::{luau_insn_a::luau_insn_a, luau_insn_op::luau_insn_op},
};

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_for_n_loop(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: `pc` 依契约指向当前字节码数组内的合法 FORNLOOP 指令字；`code[pcpos]` 读 Copy 的 `Instruction`
  // (u32)，一次读取复用于 a/op/jump-target 解码，避免重复解引用。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;

  let repeat_jump_target = get_jump_target(insn, pcpos as u32);
  let loop_repeat = build.block_at_inst(repeat_jump_target as u32);
  let op = LuauOpcode::from(luau_insn_op(insn) as u8);
  let op_length = get_op_length(op);
  let loop_exit = build.block_at_inst((pcpos + op_length) as u32);

  CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
  // 不变式（对齐 cpp LUAU_ASSERT + back()）：FORNLOOP 只能译自 FORNPREP 已压栈的循环体，
  // 栈空即前端字节码配对被破坏，属编译器内部不变量。
  let loop_info = build
    .numeric_loop_stack
    .last()
    .copied()
    .expect("numeric_loop_stack 必非空：FORNLOOP 前必有 FORNPREP 压栈（cpp 同源断言）");
  let step_k = loop_info.step;

  if repeat_jump_target != loop_info.startpc {
    let pcpos_op = build.const_uint(pcpos as u32);
    build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos_op);
  }

  if LuauBackedgeHeapCheck.get() {
    build.inst_ir_cmd(IrCmd::CheckGc);
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

    let (lhs, rhs) = if step_n > 0.0 {
      (idx, limit)
    } else {
      (limit, idx)
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::JumpCmpNum,
      lhs,
      rhs,
      cond_op,
      loop_repeat,
      loop_exit,
    );
  }

  if build.is_internal_block(loop_exit) {
    build.begin_block(loop_exit);
  }
}
