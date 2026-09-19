use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_inst::BcInst, bc_op::BcOp, bytecode_graph_serializer::BytecodeGraphSerializer,
    jump_info::JumpInfo,
  },
};

impl<'a> BytecodeGraphSerializer<'a> {
  pub fn record_jump(&mut self, insn: &mut BcInst, index: u8) {
    LUAU_ASSERT!(index < insn.ops.len() as u8);
    let inp: BcOp = insn.ops[index as usize];
    LUAU_ASSERT!(inp.kind == BcOpKind::Block);
    self.jumps.push(JumpInfo {
      op: insn.op,
      instruction_pc: self.bcb.get_instruction_count() as u32,
      target_block: inp,
    });
  }
}
