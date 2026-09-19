use core::mem;

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{bc_function::BcFunction, bc_imm::BcImm, bc_op::BcOp},
};

impl BcFunction {
  pub fn add_imm(&mut self, kind: BcImmKind) -> BcOp {
    let imm = BcImm {
      kind,
      // SAFETY：BcImmValue 各分支(bool/i32/u32)全零位型均合法，
      // kind 未限定活跃字段前仅作占位，读取必先经 kind 判别
      value: unsafe { mem::zeroed() },
    };
    self.immediates.push(imm);
    BcOp::bc_op_bc_op_kind_u32(BcOpKind::Imm, (self.immediates.len() - 1) as u32)
  }
}
