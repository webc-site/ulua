use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{is_fast_call::is_fast_call, is_skip_c::is_skip_c},
  macros::{luau_assert::LUAU_ASSERT, luau_insn_c::luau_insn_c, luau_insn_op::luau_insn_op},
};

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn patch_skip_c(&mut self, jump_label: usize, target_label: usize) -> bool {
    LUAU_ASSERT!(jump_label < self.insns.len());

    let jump_insn = self.insns[jump_label];
    {
      let _ = jump_insn;
    }

    let op = LuauOpcode::from(luau_insn_op(jump_insn) as u8);
    LUAU_ASSERT!(is_skip_c(op) || is_fast_call(op));
    LUAU_ASSERT!(luau_insn_c(jump_insn) == 0);

    let offset = (target_label as i32) - (jump_label as i32) - 1;

    if (offset as u8) as i32 != offset {
      return false;
    }

    self.insns[jump_label] |= (offset as u32) << 24;
    true
  }
}
