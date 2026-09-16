use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{is_fast_call::isFastCall, is_skip_c::isSkipC},
  macros::{luau_assert::LUAU_ASSERT, luau_insn_c::LUAU_INSN_C, luau_insn_op::LUAU_INSN_OP},
};

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn patch_skip_c(&mut self, jump_label: usize, target_label: usize) -> bool {
    LUAU_ASSERT!(jump_label < self.insns.len());

    let jump_insn = self.insns[jump_label];
    {
      let _ = jump_insn;
    }

    let op = LuauOpcode::from(LUAU_INSN_OP(jump_insn) as u8);
    LUAU_ASSERT!(isSkipC(op) || isFastCall(op));
    LUAU_ASSERT!(LUAU_INSN_C(jump_insn) == 0);

    let offset = (target_label as i32) - (jump_label as i32) - 1;

    if (offset as u8) as i32 != offset {
      return false;
    }

    self.insns[jump_label] |= (offset as u32) << 24;
    true
  }
}
