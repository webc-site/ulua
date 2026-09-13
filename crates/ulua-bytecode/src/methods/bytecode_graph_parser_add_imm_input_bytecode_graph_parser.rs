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
  ///
  /// `inst` must be a valid, aligned, non-null pointer to a `BcInst`.
  pub unsafe fn add_imm_input_bc_inst_bool(&mut self, inst: *mut BcInst, value: bool) {
    let inst = unsafe { &mut *inst };
    let index = self
      .func
      .immediates
      .iter()
      .position(|imm| imm.kind == BcImmKind::Boolean && unsafe { imm.value.value_boolean } == value)
      .unwrap_or_else(|| {
        let idx = self.func.immediates.len();
        self.func.immediates.push(BcImm {
          kind: BcImmKind::Boolean,
          value: BcImmValue {
            value_boolean: value,
          },
        });
        idx
      });

    let op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, index as u32);
    inst.ops.push_back(op);
  }
}
