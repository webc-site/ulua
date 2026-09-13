use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer};

impl<'a> BytecodeGraphSerializer<'a> {
  pub fn get_reg_input(&mut self, insn: &mut BcInst, index: u8) -> u8 {
    LUAU_ASSERT!((index as usize) < insn.ops.len());
    self.get_register(insn.ops[index as usize])
  }
}
