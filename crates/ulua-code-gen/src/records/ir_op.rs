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
  /// `IrOpKind` 最大合法 discriminant（当前仅 0..=KIND_MAX，掩码仍可能造出更大值）
  const KIND_MAX: u32 = IrOpKind::VmExit as u32;

  pub fn kind(&self) -> IrOpKind {
    // KIND_MASK = 0xF 允许 0..=15，而 IrOpKind 仅 0..=9：非法值兜底为 None，避免构造非法 discriminant
    let raw = self.kind_and_index & Self::KIND_MASK;
    debug_assert!(raw <= Self::KIND_MAX, "IrOp kind 掩码值 {raw} 超出 IrOpKind 范围");
    match raw {
      0 => IrOpKind::None,
      1 => IrOpKind::Undef,
      2 => IrOpKind::Constant,
      3 => IrOpKind::Condition,
      4 => IrOpKind::Inst,
      5 => IrOpKind::Block,
      6 => IrOpKind::VmReg,
      7 => IrOpKind::VmConst,
      8 => IrOpKind::VmUpvalue,
      9 => IrOpKind::VmExit,
      _ => IrOpKind::None,
    }
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

#[cfg(test)]
mod tests {
  use super::IrOp;
  use crate::enums::ir_op_kind::IrOpKind;

  #[test]
  fn kind_and_index_round_trip_all_variants() {
    let kinds = [
      IrOpKind::None,
      IrOpKind::Undef,
      IrOpKind::Constant,
      IrOpKind::Condition,
      IrOpKind::Inst,
      IrOpKind::Block,
      IrOpKind::VmReg,
      IrOpKind::VmConst,
      IrOpKind::VmUpvalue,
      IrOpKind::VmExit,
    ];
    for (i, kind) in kinds.iter().copied().enumerate() {
      let op = IrOp::ir_op_kind_u32(kind, i as u32);
      assert_eq!(op.kind(), kind);
      assert_eq!(op.index(), i as u32, "kind {kind:?} 的 index 被掩码污染");
    }
    assert_eq!(IrOp::default().kind(), IrOpKind::None);
  }

  #[test]
  #[cfg(debug_assertions)]
  #[should_panic(expected = "超出 IrOpKind 范围")]
  fn kind_rejects_mask_value_beyond_last_variant() {
    let _ = IrOp { kind_and_index: 10 }.kind();
  }
}
