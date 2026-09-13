use crate::records::{bc_inst::BcInst, bc_inst_eq::BcInstEq};

impl BcInstEq {
  pub fn call(&self, a: &BcInst, b: &BcInst) -> bool {
    a.op == b.op && a.ops == b.ops
  }
}
