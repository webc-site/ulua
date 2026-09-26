use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_b(insn: u32) -> u32 {
  Instruction(insn).b() as u32
}
