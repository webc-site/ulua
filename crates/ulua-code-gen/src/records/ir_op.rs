use core::mem::transmute;

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::enums::ir_op_kind::IrOpKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IrOp {
  pub(crate) kind_and_index: u32,
}

impl IrOp {
  pub(crate) const KIND_MASK: u32 = 0xF;
  pub(crate) const INDEX_SHIFT: u32 = 4;

  pub fn kind(&self) -> IrOpKind {
    unsafe { transmute(self.kind_and_index & Self::KIND_MASK) }
  }

  pub fn index(&self) -> u32 {
    self.kind_and_index >> Self::INDEX_SHIFT
  }
}

impl Default for IrOp {
  fn default() -> Self {
    Self {
      kind_and_index: IrOpKind::None as u32,
    }
  }
}

impl DenseDefault for IrOp {
  fn dense_default() -> Self {
    Self::default()
  }
}
