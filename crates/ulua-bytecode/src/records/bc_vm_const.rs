use core::fmt::{self, Debug, Formatter};

use crate::enums::bc_vm_const_kind::BcVmConstKind;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BcVmConst {
  pub kind: BcVmConstKind,
  pub value: BcVmConstValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union BcVmConstValue {
  pub value_boolean: bool,
  pub value_number: f64,
  pub value_vector: [f32; 4],
  pub value_string: &'static str,
  pub value_import: u32,
  pub value_table: u32,
  pub value_closure: u32,
  pub value_integer: i64,
}

impl Debug for BcVmConstValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str("BcVmConstValue(..)")
  }
}
