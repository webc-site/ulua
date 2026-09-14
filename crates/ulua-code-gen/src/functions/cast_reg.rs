use crate::{enums::kind_a_64::KindA64, records::register_a_64::RegisterA64};

pub const fn cast_reg(kind: KindA64, reg: RegisterA64) -> RegisterA64 {
  // Since RegisterA64::kind() and RegisterA64::index() are not marked const in their dependency card,
  // we must access the bits directly using the masks and shifts provided in the record's public API.
  let reg_kind_bits = reg.bits & RegisterA64::KIND_MASK;
  let kind_bits = kind as u8;

  // CODEGEN_ASSERT is not usable in const fn contexts because it expands to
  // non-const calls (assertCallHandler, intrinsics).
  // We use a standard assert! which is supported in const fns since Rust 1.57.
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
