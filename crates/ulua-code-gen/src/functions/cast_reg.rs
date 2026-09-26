use crate::{enums::kind_a_64::KindA64, records::register_a_64::RegisterA64};

pub const fn cast_reg(kind: KindA64, reg: RegisterA64) -> RegisterA64 {
  // 依赖卡中 RegisterA64::kind() 与 index() 未标 const，
  // 因此这里用 record public API 提供的掩码和移位直接访问 bits。
  let reg_kind_bits = reg.bits & RegisterA64::KIND_MASK;
  let kind_bits = kind as u8;

  // CODEGEN_ASSERT 不能用于 const fn 上下文：它展开为非 const
  // 调用（assertCallHandler、intrinsics）。
  // 改用标准 assert!，Rust 1.57 起在 const fn 中可用。
  assert!(kind_bits != reg_kind_bits);
  assert!(kind_bits != KindA64::None as u8 && reg_kind_bits != KindA64::None as u8);
  assert!(
    (kind_bits == KindA64::W as u8 || kind_bits == KindA64::X as u8)
      == (reg_kind_bits == KindA64::W as u8 || reg_kind_bits == KindA64::X as u8)
  );

  let reg_index_bits = reg.bits & RegisterA64::INDEX_MASK;

  RegisterA64 {
    bits: kind_bits | reg_index_bits,
  }
}
