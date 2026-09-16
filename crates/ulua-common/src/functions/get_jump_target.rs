mod _inner {
  use crate::{
    enums::luau_opcode::LuauOpcode,
    functions::{is_fast_call::is_fast_call, is_jump_d::is_jump_d, is_skip_c::is_skip_c},
    macros::{
      luau_insn_c::luau_insn_c, luau_insn_d::luau_insn_d, luau_insn_e::luau_insn_e,
      luau_insn_op::luau_insn_op,
    },
  };

  #[inline]
  pub fn get_jump_target(insn: u32, pc: u32) -> i32 {
    // `LuauOpcode::from` 把越界操作码字节钳到 `LopNop`；NOP 不属于任何
    // 跳转/调用族，最终落到兜底 `-1`，与 C++ switch default 语义一致。
    let op = LuauOpcode::from(luau_insn_op(insn) as u8);

    if is_jump_d(op) {
      (pc as i32).wrapping_add(luau_insn_d(insn)).wrapping_add(1)
    } else if is_fast_call(op) {
      (pc as i32)
        .wrapping_add(luau_insn_c(insn) as i32)
        .wrapping_add(2)
    } else if is_skip_c(op) && luau_insn_c(insn) != 0 {
      (pc as i32)
        .wrapping_add(luau_insn_c(insn) as i32)
        .wrapping_add(1)
    } else if op == LuauOpcode::LOP_JUMPX {
      (pc as i32).wrapping_add(luau_insn_e(insn)).wrapping_add(1)
    } else {
      -1
    }
  }
}

pub use _inner::{get_jump_target, get_jump_target as getJumpTarget};
