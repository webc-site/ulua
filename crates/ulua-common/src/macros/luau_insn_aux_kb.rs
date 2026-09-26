use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_aux_kb(aux: u32) -> u32 {
  Instruction(aux).aux_kb()
}
