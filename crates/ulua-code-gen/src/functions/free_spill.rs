use crate::enums::kind_a_64::KindA64;

pub fn free_spill(free: &mut u64, kind: KindA64, slot: u8) {
  // qN registers use two consecutive slots
  let mask = (if kind == KindA64::Q { 3u64 } else { 1u64 }) << (slot as u64);

  // Equivalent to CODEGEN_ASSERT!((free & mask) == 0) without relying on the CODEGEN_ASSERT macro.
  assert!((*free & mask) == 0);

  *free |= mask;
}
