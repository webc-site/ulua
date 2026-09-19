use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_c::luau_insn_c, luau_insn_d::luau_insn_d, luau_insn_e::luau_insn_e,
    luau_insn_op::luau_insn_op,
  },
};

use crate::functions::{is_fast_call::is_fast_call, is_jump_d::is_jump_d, is_skip_c::is_skip_c};

pub fn get_jump_target(insn: u32, pc: u32) -> i32 {
  let op_val = luau_insn_op(insn) as u8;
  // opcode 字节钳制转换：越界（损坏字节码）落 LopNop，与 C++ default → -1 一致，避免 UB
  let op: LuauOpcode = LuauOpcode::from(op_val);

  if is_jump_d(op) {
    (pc as i32) + luau_insn_d(insn) + 1
  } else if is_fast_call(op) {
    (pc as i32) + (luau_insn_c(insn) as i32) + 2
  } else if is_skip_c(op) && luau_insn_c(insn) != 0 {
    (pc as i32) + (luau_insn_c(insn) as i32) + 1
  } else if op == LuauOpcode::LOP_JUMPX {
    (pc as i32) + luau_insn_e(insn) + 1
  } else {
    -1
  }
}
