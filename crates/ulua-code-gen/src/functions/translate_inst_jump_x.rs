use ulua_common::macros::luau_insn_e::LUAU_INSN_E;

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn translate_inst_jump_x(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
  let interrupt_op = build.const_uint(pcpos as u32);
  build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_op);

  let jump_target = pcpos + 1 + LUAU_INSN_E(unsafe { *pc });
  let jump_block = build.block_at_inst(jump_target as u32);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, jump_block);
}
