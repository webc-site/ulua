use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addVmConstInput(BcRef<BcInst>, uint32_t)`。
  pub fn add_vm_const_input(&mut self, inst: BcOp, idx: u32) {
    LUAU_ASSERT!(idx < self.func.constants.len() as u32);
    self
      .func
      .inst_op(inst)
      .ops
      .push_back(BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmConst, idx));
  }
}
