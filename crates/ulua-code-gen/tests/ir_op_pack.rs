//! `IrOp` 的 kind+index 打包往返（cpp `Ir.h` 里 `IrOp::kind`/`index` 是
//! 位域，移植压成单个 `u32`：低 4 位 kind、高 28 位 index）。
//!
//! 为什么钉：打包用 `KIND_MASK = 0xF` 比实际变体数（0..=9）宽，index 一旦
//! 左移量写错就会被 kind 掩码污染，读回来的 kind/index 双双错位且不报错。

use ulua_code_gen::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

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
  let _ = IrOp::from_raw_kind_and_index(10).kind();
}
