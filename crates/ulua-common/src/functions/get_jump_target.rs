mod _inner {
  use core::mem::transmute;

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
    let op_u8 = luau_insn_op(insn) as u8;
    if op_u8 >= LuauOpcode::LOP__COUNT as u8 {
      return -1;
    }

    // 安全性：`LuauOpcode` 是 `#[repr(u8)]`，判别值在 `0..LOP__COUNT` 之间连续，
    // 上方的边界检查确保 `op_u8` 是有效的判别值，因此 transmute 是安全的。
    let op: LuauOpcode = unsafe { transmute(op_u8) };

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
