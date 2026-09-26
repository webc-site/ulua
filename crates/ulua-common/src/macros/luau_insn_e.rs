use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_e(insn: u32) -> i32 {
  Instruction(insn).e()
}
