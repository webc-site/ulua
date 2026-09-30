use crate::{
  enums::luau_opcode::LuauOpcode,
  functions::{is_fast_call::is_fast_call, is_jump_d::is_jump_d, is_skip_c::is_skip_c},
  records::instruction::Instruction,
};

#[inline]
pub fn get_jump_target(insn: u32, pc: u32) -> i32 {
  // `Instruction::luau_opcode` 把越界操作码字节钳到 `LopNop`；NOP 不属于任何
  // 跳转/调用族，最终落到兜底 `-1`，与 C++ switch default 语义一致。
  let insn = Instruction(insn);
  let op = insn.luau_opcode();

  if is_jump_d(op) {
    (pc as i32).wrapping_add(insn.d() as i32).wrapping_add(1)
  } else if is_fast_call(op) {
    (pc as i32).wrapping_add(insn.c() as i32).wrapping_add(2)
  } else if is_skip_c(op) && insn.c() != 0 {
    (pc as i32).wrapping_add(insn.c() as i32).wrapping_add(1)
  } else if op == LuauOpcode::LOP_JUMPX {
    (pc as i32).wrapping_add(insn.e()).wrapping_add(1)
  } else {
    -1
  }
}
