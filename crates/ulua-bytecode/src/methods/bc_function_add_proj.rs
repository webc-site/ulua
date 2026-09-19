use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_op::BcOp, bc_proj::BcProj},
};

impl BcFunction {
  pub fn add_proj(&mut self, op: BcOp, index: u32) -> BcOp {
    self.projections.push(BcProj { op, index });
    BcOp::bc_op_bc_op_kind_u32(BcOpKind::Proj, (self.projections.len() - 1) as u32)
  }
}
