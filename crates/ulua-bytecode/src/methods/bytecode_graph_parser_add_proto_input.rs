use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_inst::BcInst, bc_op::BcOp, bytecode_graph_parser::BytecodeGraphParser},
};

impl<'a> BytecodeGraphParser<'a> {
  /// # Safety
  ///
  /// `inst` must be a valid, aligned, non-null pointer to a `BcInst`.
  pub unsafe fn add_proto_input(&mut self, inst: *mut BcInst, idx: u32) {
    let inst = unsafe { &mut *inst };
    inst
      .ops
      .push_back(BcOp::bc_op_bc_op_kind_u32(BcOpKind::VmProto, idx));
  }
}
