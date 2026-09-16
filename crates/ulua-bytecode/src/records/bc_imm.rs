use core::{
  fmt::{self, Debug, Formatter},
  hash::{Hash, Hasher},
};

use crate::enums::bc_imm_kind::BcImmKind;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BcImm {
  pub kind: BcImmKind,
  pub value: BcImmValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union BcImmValue {
  pub value_boolean: bool,
  pub value_int: i32,
  pub value_import: u32,
}

impl Debug for BcImmValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("BcImmValue").finish_non_exhaustive()
  }
}

impl PartialEq for BcImmValue {
  fn eq(&self, _other: &Self) -> bool {
    // Safety: Union equality is context-dependent on BcImm.kind
    unsafe { self.value_import == _other.value_import }
  }
}

impl Eq for BcImmValue {}

impl Hash for BcImmValue {
  fn hash<H: Hasher>(&self, state: &mut H) {
    unsafe { self.value_import.hash(state) }
  }
}
