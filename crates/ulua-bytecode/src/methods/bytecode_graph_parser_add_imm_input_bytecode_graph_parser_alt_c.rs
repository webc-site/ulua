use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_imm::{BcImm, BcImmValue},
    bc_op::BcOp,
    bytecode_graph_parser::BytecodeGraphParser,
  },
};

impl<'a> BytecodeGraphParser<'a> {
  /// cpp `addImmInput(BcRef<BcInst>, uint32_t)`（`BytecodeGraphParser.h:420-427`）：
  /// Import 型立即数，同样每条操作数独占一条 `immediates` 记录。
  pub(crate) fn add_imm_input_bc_inst_u32(&mut self, inst: BcOp, value: u32) {
    self.func.immediates.push(BcImm {
      kind: BcImmKind::Import,
      value: BcImmValue {
        value_import: value,
      },
    });

    let op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, (self.func.immediates.len() - 1) as u32);
    self.func.inst_op(inst).ops.push_back(op);
  }
}
