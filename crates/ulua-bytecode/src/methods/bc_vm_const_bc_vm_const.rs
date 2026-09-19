use crate::{
  enums::bc_vm_const_kind::BcVmConstKind,
  records::bc_vm_const::{BcVmConst, BcVmConstValue},
};

impl BcVmConst {
  pub fn new() -> Self {
    Self {
      kind: BcVmConstKind::Nil,
      value: BcVmConstValue { value_integer: 0 },
    }
  }
}

impl Default for BcVmConst {
  fn default() -> Self {
    Self::new()
  }
}
