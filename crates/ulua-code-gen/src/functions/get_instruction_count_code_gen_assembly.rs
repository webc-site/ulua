use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::get_op_length::get_op_length,
  macros::luau_insn_ops::luau_insn_op,
};

use crate::type_aliases::instruction_ir_builder::Instruction;

pub fn get_instruction_count(insns: &[Instruction]) -> u32 {
  let mut count: u32 = 0;
  let mut i: usize = 0;

  while i < insns.len() {
    count += 1;
    let op = luau_insn_op(insns[i]) as u8;
    let op_enum = LuauOpcode::from(op);
    let len = get_op_length(op_enum).max(1) as usize;
    i += len;
  }

  count
}
