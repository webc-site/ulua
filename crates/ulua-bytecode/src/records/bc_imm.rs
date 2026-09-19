use core::{
  fmt::{self, Debug, Formatter},
  hash::{Hash, Hasher},
};

use crate::enums::bc_imm_kind::BcImmKind;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

// 联合体的活动字段由 `kind` 决定：只有按 kind 读取活动分支才不是未定义行为，
// 因此 PartialEq/Eq/Hash 手写而非 derive（同 `BcVmConst`）。
impl PartialEq for BcImm {
  fn eq(&self, other: &Self) -> bool {
    if self.kind != other.kind {
      return false;
    }
    match self.kind {
      BcImmKind::Boolean => unsafe { self.value.value_boolean == other.value.value_boolean },
      BcImmKind::Int => unsafe { self.value.value_int == other.value.value_int },
      BcImmKind::Import => unsafe { self.value.value_import == other.value.value_import },
    }
  }
}

impl Eq for BcImm {}

impl Hash for BcImm {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.kind.hash(state);
    match self.kind {
      BcImmKind::Boolean => unsafe { self.value.value_boolean }.hash(state),
      BcImmKind::Int => unsafe { self.value.value_int }.hash(state),
      BcImmKind::Import => unsafe { self.value.value_import }.hash(state),
    }
  }
}
