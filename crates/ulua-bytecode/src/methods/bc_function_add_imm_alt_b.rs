use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_imm::BcImm, bc_op::BcOp},
};

impl BcFunction {
  /// cpp `BcFunction::addImm(const BcImm&)`：按现值追加立即数。
  pub fn add_imm_alt_b(&mut self, imm: BcImm) -> BcOp {
    self.immediates.push(imm);
    BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, (self.immediates.len() - 1) as u32)
  }
}
