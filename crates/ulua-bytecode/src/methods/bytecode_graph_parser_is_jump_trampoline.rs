use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::get_jump_target::get_jump_target,
  macros::luau_insn_op::LUAU_INSN_OP,
};

use crate::{
  records::bytecode_graph_parser::BytecodeGraphParser, type_aliases::instruction::Instruction,
};

impl<'a> BytecodeGraphParser<'a> {
  pub fn is_jump_trampoline(&self, pc: u32, code: &[Instruction]) -> bool {
    let pc = pc as usize;
    if pc >= code.len()
      || LuauOpcode::from((LUAU_INSN_OP(code[pc]) & 0xff) as u8) != LuauOpcode::LOP_JUMP
    {
      return false;
    }

    if pc + 1 >= code.len() {
      return false;
    }

    let op1 = LuauOpcode::from((LUAU_INSN_OP(code[pc + 1]) & 0xff) as u8);
    if op1 != LuauOpcode::LOP_JUMPX {
      return false;
    }

    if pc + 2 >= code.len() {
      return false;
    }

    let target = get_jump_target(code[pc + 2], (pc + 2) as u32) as u32;
    target == (pc + 1) as u32
  }
}
