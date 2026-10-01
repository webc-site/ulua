use std::vec::Vec;

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::enums::bc_op_kind::BcOpKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BcOp {
  pub kind: BcOpKind,
  pub index: u32,
}

impl BcOp {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with(kind: BcOpKind, index: u32) -> Self {
    Self { kind, index }
  }

  /// 图各 arena（blocks/instructions/phis/projections/constants/immediates）
  /// 「追加并回填句柄」的唯一来源：push 后取 `len - 1` 作 index，等价 cpp
  /// `addXxx` 系列尾部的 `uint32_t(size - 1)`，各 `BcFunction::add_*` 只保留
  /// 值构造差异。
  pub(crate) fn pushed<T>(arena: &mut Vec<T>, kind: BcOpKind, value: T) -> Self {
    arena.push(value);
    Self::with(kind, (arena.len() - 1) as u32)
  }
}

impl DenseDefault for BcOp {
  fn dense_default() -> Self {
    Self::default()
  }
}
