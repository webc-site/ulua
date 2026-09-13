use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{luau_insn_a::LUAU_INSN_A, luau_insn_op::LUAU_INSN_OP},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_condition::IrCondition, ir_op_kind::IrOpKind},
  functions::{get_jump_target::get_jump_target, get_op_length::get_op_length},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_for_n_prep(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let ra = LUAU_INSN_A(unsafe { *pc }) as u8;

  let op = LuauOpcode::from(LUAU_INSN_OP(unsafe { *pc }) as u8);
  let loop_start = build.block_at_inst((pcpos + get_op_length(op)) as u32);
  let loop_exit = build.block_at_inst(get_jump_target(unsafe { *pc }, pcpos as u32) as u32);

  CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
  let step_k = build.numeric_loop_stack.last().unwrap().step;

  let reg_limit = build.vm_reg(ra);
  let tag_limit = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_limit);
  let number_tag = build.const_tag(LuaType::Number as u8);
  let exit = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_limit, number_tag, exit);

  let reg_idx = build.vm_reg(ra + 2);
  let tag_idx = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_idx);
  let number_tag = build.const_tag(LuaType::Number as u8);
  let exit = build.vm_exit(pcpos as u32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_idx, number_tag, exit);

  let reg_limit = build.vm_reg(ra);
  let limit = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_limit);
  let reg_idx = build.vm_reg(ra + 2);
  let idx = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_idx);

  if step_k.kind() == IrOpKind::Undef {
    let reg_step = build.vm_reg(ra + 1);
    let tag_step = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_step);
    let number_tag = build.const_tag(LuaType::Number as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tag_step, number_tag, exit);

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

    if step_n > 0.0 {
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpNum,
        idx,
        limit,
        cond,
        loop_exit,
        loop_start,
      );
    } else {
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JumpCmpNum,
        limit,
        idx,
        cond,
        loop_exit,
        loop_start,
      );
    }
  }

  if build.is_internal_block(loop_start) {
    build.begin_block(loop_start);
  }

  build.interrupt_requested = true;
}
