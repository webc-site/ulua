use ulua_common::records::dense_hash_table::DenseDefault;

use crate::enums::ir_op_kind::IrOpKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IrOp {
  pub(crate) kind_and_index: u32,
}

// DenseHashMap<u32, IrOp> 以 IrOp 为值类型（const_prop_state 的 inst_value
// 槽位占位值机制），DenseDefault 为其必需约束——勿因键位 grep 误判孤儿
impl DenseDefault for IrOp {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl IrOp {
  pub const fn from_raw_kind_and_index(kind_and_index: u32) -> Self {
    Self { kind_and_index }
  }

  pub(crate) const KIND_MASK: u32 = 0xF;
  pub(crate) const INDEX_SHIFT: u32 = 4;
  /// `IrOpKind` 最大合法 discriminant（当前仅 0..=KIND_MAX，掩码仍可能造出更大值）
  const KIND_MAX: u32 = IrOpKind::VmExit as u32;

  pub fn kind(&self) -> IrOpKind {
    let raw = self.kind_and_index & Self::KIND_MASK;
    debug_assert!(
      raw <= Self::KIND_MAX,
      "IrOp kind 掩码值 {raw} 超出 IrOpKind 范围"
    );
    IrOpKind::from_repr(raw).unwrap_or(IrOpKind::None)
  }

  pub fn index(&self) -> u32 {
    self.kind_and_index >> Self::INDEX_SHIFT
  }

  pub fn new() -> IrOp {
    IrOp::ir_op_kind_u32(IrOpKind::None, 0)
  }

  pub fn ir_op_kind_u32(kind: IrOpKind, index: u32) -> IrOp {
    IrOp {
      kind_and_index: (kind as u32) | (index << IrOp::INDEX_SHIFT),
    }
  }

  pub fn ir_op_ir_op_kind_u32(kind: IrOpKind, index: u32) -> IrOp {
    IrOp::ir_op_kind_u32(kind, index)
  }

  #[inline]
  pub fn ir_op_operator_eq(&self, rhs: IrOp) -> bool {
    self.kind() == rhs.kind() && self.index() == rhs.index()
  }
}

impl Default for IrOp {
  fn default() -> Self {
    Self {
      kind_and_index: IrOpKind::None as u32,
    }
  }
}

impl PartialEq<IrOp> for &IrOp {
  #[inline]
  fn eq(&self, other: &IrOp) -> bool {
    self.kind_and_index == other.kind_and_index
  }
}
