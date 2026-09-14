use core::mem::transmute;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_c::LUAU_INSN_C, luau_insn_d::LUAU_INSN_D, luau_insn_e::LUAU_INSN_E,
    luau_insn_op::LUAU_INSN_OP,
  },
};

use crate::functions::{is_fast_call::is_fast_call, is_jump_d::is_jump_d, is_skip_c::is_skip_c};

pub fn get_jump_target(insn: u32, pc: u32) -> i32 {
  let op_val = LUAU_INSN_OP(insn) as u8;
  let op: LuauOpcode = unsafe { transmute(op_val) };

  if is_jump_d(op) {
    (pc as i32) + LUAU_INSN_D(insn) + 1
  } else if is_fast_call(op) {
    (pc as i32) + (LUAU_INSN_C(insn) as i32) + 2
  } else if is_skip_c(op) && LUAU_INSN_C(insn) != 0 {
    (pc as i32) + (LUAU_INSN_C(insn) as i32) + 1
  } else if op == LuauOpcode::LOP_JUMPX {
    (pc as i32) + LUAU_INSN_E(insn) + 1
  } else {
    -1
  }
}
