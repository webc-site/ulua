use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_inst::BcInst, bytecode_graph_serializer::BytecodeGraphSerializer},
};

impl<'a> BytecodeGraphSerializer<'a> {
  pub fn get_proto_input(&mut self, insn: &mut BcInst, index: u8) -> u16 {
    LUAU_ASSERT!(index < insn.ops.len() as u8);
    let inp = insn.ops[index as usize];
    LUAU_ASSERT!(inp.kind == BcOpKind::VmProto);

    if inp.index > 0xffff {
      self.error = true;
    }

    inp.index as u16
  }
}
