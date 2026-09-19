use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{
    bc_inst::BcInst, bc_op::BcOp, bytecode_builder::K_MAX_UPVALUE_COUNT,
    bytecode_graph_serializer::BytecodeGraphSerializer,
  },
};

impl<'a> BytecodeGraphSerializer<'a> {
  /// cpp `getUpvalInput`（BytecodeGraphSerializer.h:258-275）：upvalue 索引
  /// 超出函数 nups 或 `kMaxUpvalueCount` 时置 error。
  pub fn get_upval_input(&mut self, insn: &mut BcInst, index: u8) -> u8 {
    LUAU_ASSERT!((index as usize) < insn.ops.len());
    let inp: BcOp = insn.ops[index as usize];
    LUAU_ASSERT!(inp.kind == BcOpKind::VmUpvalue);

    if inp.index >= self.func.nups as u32 {
      LUAU_ASSERT!(
        false,
        "upvalue reference overflows the function upvalue count"
      );
      self.error = true;
    }

    if inp.index >= K_MAX_UPVALUE_COUNT {
      self.error = true;
    }

    LUAU_ASSERT!(inp.index < self.func.nups as u32);
    inp.index as u8
  }
}
