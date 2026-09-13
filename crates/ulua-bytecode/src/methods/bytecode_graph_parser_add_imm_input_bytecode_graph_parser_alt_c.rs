use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_imm::{BcImm, BcImmValue},
    bc_inst::BcInst,
    bc_op::BcOp,
    bytecode_graph_parser::BytecodeGraphParser,
  },
};

impl<'a> BytecodeGraphParser<'a> {
  /// # Safety
  /// `inst` must be a valid, non-null pointer to an initialized `BcInst`.
  pub(crate) unsafe fn add_imm_input_bc_inst_u32(&mut self, inst: *mut BcInst, value: u32) {
    let inst = unsafe { &mut *inst };
    self.func.immediates.push(BcImm {
      kind: BcImmKind::Import,
      value: BcImmValue {
        value_import: value,
      },
    });

    let op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, (self.func.immediates.len() - 1) as u32);
    inst.ops.push_back(op);
  }
}
