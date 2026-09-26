use ulua_common::{
  fflag::LuauBackedgeHeapCheck,
  macros::{luau_insn_d::luau_insn_d, luau_insn_e::luau_insn_e},
};

use crate::{
  enums::ir_cmd::IrCmd, records::ir_builder::IrBuilder,
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_jump(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  let target = pcpos + 1 + luau_insn_d(code[pcpos as usize]);
  let block = build.block_at_inst(target as u32);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
}

#[inline]
fn translate_inst_jump_with_check(build: &mut IrBuilder, pcpos: i32, offset: i32) {
  let interrupt_op = build.const_uint(pcpos as u32);
  build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_op);

  if LuauBackedgeHeapCheck.get() {
    build.inst_ir_cmd(IrCmd::CheckGc);
  }

  let target_block = build.block_at_inst((pcpos + 1 + offset) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, target_block);
}

pub fn translate_inst_jump_back(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_inst_jump_with_check(build, pcpos, luau_insn_d(code[pcpos as usize]));
}

pub fn translate_inst_jump_x(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_inst_jump_with_check(build, pcpos, luau_insn_e(code[pcpos as usize]));
}
