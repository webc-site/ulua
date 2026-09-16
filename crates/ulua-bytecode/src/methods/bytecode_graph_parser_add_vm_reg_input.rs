use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  methods::bytecode_graph_parser_is_unreachable::bytecode_graph_parser_is_unreachable,
  records::{bc_inst::BcInst, bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphParser<'a> {
  /// # Safety
  ///
  /// `inst` must be a valid, aligned, non-null pointer to a `BcInst`.
  pub unsafe fn add_vm_reg_input(&mut self, inst: *mut BcInst, reg: Reg) {
    let inst = unsafe { &mut *inst };
    let source = self.find_producer_bc_op_reg(self.current_block, reg);
    if source.is_none() && bytecode_graph_parser_is_unreachable(self, self.current_block) {
      inst
        .ops
        .push_back(BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmReg, reg as u32));
      return;
    }
    LUAU_ASSERT!(source.is_some());
    inst.ops.push_back(source.unwrap());
  }
}
