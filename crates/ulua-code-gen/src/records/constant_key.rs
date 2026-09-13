use core::hash::{Hash, Hasher};

use crate::enums::ir_const_kind::IrConstKind;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConstantKey {
  pub(crate) kind: IrConstKind,
  pub(crate) value: u64,
}

impl PartialEq for ConstantKey {
  fn eq(&self, other: &Self) -> bool {
    self.kind == other.kind && self.value == other.value
  }
}

impl Hash for ConstantKey {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.kind.hash(state);
    self.value.hash(state);
  }
}

impl Eq for ConstantKey {}

impl Default for ConstantKey {
  fn default() -> Self {
    Self {
      kind: IrConstKind::Int,
      value: 0,
    }
  }
}
