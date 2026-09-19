use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addEmptyInput(BcRef<BcInst>)`：占位输入，`kind` 为 `None`。
  pub fn add_empty_input(&mut self, inst: BcOp) {
    self
      .func
      .inst_op(inst)
      .ops
      .push_back(BcOp::bc_op_bc_op_kind_u32(BcOpKind::None, 0));
  }
}
