use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_imm_kind::BcImmKind,
  records::{bc_imm::BcImm, bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer},
};

impl<'a> BytecodeGraphSerializer<'a> {
  pub fn get_imm_int(&mut self, insn: &mut BcInst, index: u8) -> i32 {
    let imm: &mut BcImm = self.get_imm(insn, index);
    LUAU_ASSERT!(imm.kind == BcImmKind::Int);
    unsafe { imm.value.value_int }
  }
}
