use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::bc_op_kind::BcOpKind,
  methods::bytecode_graph_parser_is_unreachable::bytecode_graph_parser_is_unreachable,
  records::{bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
  type_aliases::reg::Reg,
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addVmRegInput(BcRef<BcInst>, Reg)`：解析该寄存器在当前块里的生产者并挂为输入。
  pub fn add_vm_reg_input(&mut self, inst: BcOp, reg: Reg) {
    let source = self.find_producer_bc_op_reg(self.current_block, reg);
    if source.is_none() && bytecode_graph_parser_is_unreachable(self, self.current_block) {
      self
        .func
        .inst_op(inst)
        .ops
        .push_back(BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmReg, reg as u32));
      return;
    }
    LUAU_ASSERT!(source.is_some());
    self.func.inst_op(inst).ops.push_back(source.unwrap());
  }
}
