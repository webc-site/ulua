use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_c(insn: u32) -> u32 {
  Instruction(insn).c() as u32
}
