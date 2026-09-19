use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addProtoInput(BcRef<BcInst>, uint32_t)`。
  pub fn add_proto_input(&mut self, inst: BcOp, idx: u32) {
    self
      .func
      .inst_op(inst)
      .ops
      .push_back(BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmProto, idx));
  }
}
