use crate::{enums::kind_a_64::KindA64, functions::countrz_bit_utils_alt_b::countrz_u64};

pub fn alloc_spill(free: &mut u64, kind: KindA64) -> i32 {
  // to support larger stack frames, we need to ensure qN is allocated at 16b boundary to fit in ldr/str encoding

  let mut search = *free;

  // qN registers use two consecutive slots
  if kind == KindA64::Q {
    // Make sure bit N is set only if bit N+1 is also set
    search = *free & (*free >> 1);

    // Prevent qN from allocating at stack/extra spill storage boundary (by reserving last stack slot)
    // In the original code this is (kSpillSlots - 1); we keep the same behavior with kSpillSlots=22.
    const K_SPILL_SLOTS: u32 = 22;
    search &= !(1u64 << (K_SPILL_SLOTS - 1));
  }

  let slot = countrz_u64(search);
  if slot == 64 {
    return -1;
  }

  let mask = (if kind == KindA64::Q { 3u64 } else { 1u64 }) << (slot as u64);

  // CODEGEN_ASSERT((free & mask) == mask)
  assert!((*free & mask) == mask);

  *free &= !mask;

  slot
}
