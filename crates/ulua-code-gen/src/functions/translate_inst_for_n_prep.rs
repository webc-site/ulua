use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::luau_insn_ops::{luau_insn_a, luau_insn_op},
};

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind},
  functions::translate_inst_binary::check_number_tag_guard,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_for_n_prep(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  // Safety: `pc` 依契约指向当前字节码数组内的合法 FORNPREP 指令字；`code[pcpos]` 读 Copy 的 `Instruction`
  // (u32)，一次读取复用于 a/op/jump-target 解码，避免重复解引用。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;

  let op = LuauOpcode::from(luau_insn_op(insn) as u8);
  let loop_start = build.block_at_inst((pcpos + get_op_length(op)) as u32);
  let loop_exit = build.block_at_inst(get_jump_target(insn, pcpos as u32) as u32);

  CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
  // 不变式（对齐 cpp LUAU_ASSERT + back()）：进入 FORNPREP 翻译前调用方已压入本循环的 LoopInfo，
  // 栈空即翻译流程被破坏，属编译器内部不变量。
  let step_k = build
    .numeric_loop_stack
    .last()
    .expect("numeric_loop_stack 必非空：FORNPREP 翻译入口处已压栈（cpp 同源断言）")
    .step;

  // bytecode 类型侧恒为 NUMBER 期望 → 守卫失败走 vm_exit 臂（fallback 占位丢弃不触碰）。
  check_number_tag_guard(build, ra, true, pcpos, &mut IrOp::new());
  check_number_tag_guard(build, ra + 2, true, pcpos, &mut IrOp::new());

  let reg_limit = build.vm_reg(ra);
  let limit = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_limit);
  let reg_idx = build.vm_reg(ra + 2);
  let idx = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_idx);

  if step_k.kind() == IrOpKind::Undef {
    check_number_tag_guard(build, ra + 1, true, pcpos, &mut IrOp::new());
    let reg_step = build.vm_reg(ra + 1);
    let step = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_step);

    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::JumpFornLoopCond,
      idx,
      limit,
      step,
      loop_start,
      loop_exit,
    );
  } else {
    let step_n = build.function.double_op(step_k);
    let cond = build.cond(IrCondition::NotLessEqual);
    let (lhs, rhs) = if step_n > 0.0 {
      (idx, limit)
    } else {
      (limit, idx)
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::JumpCmpNum,
      lhs,
      rhs,
      cond,
      loop_exit,
      loop_start,
    );
  }

  if build.is_internal_block(loop_start) {
    build.begin_block(loop_start);
  }

  build.interrupt_requested = true;
}
