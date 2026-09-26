use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_d(insn: u32) -> i32 {
  Instruction(insn).d() as i32
}
