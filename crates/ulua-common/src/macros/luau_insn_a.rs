use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_a(insn: u32) -> u32 {
  Instruction(insn).a() as u32
}
