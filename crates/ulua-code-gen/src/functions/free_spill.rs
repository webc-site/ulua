use crate::enums::kind_a_64::KindA64;

pub fn free_spill(free: &mut u64, kind: KindA64, slot: u8) {
  // qN 寄存器占用两个相邻槽位
  let mask = (if kind == KindA64::Q { 3u64 } else { 1u64 }) << (slot as u64);

  // 等价于 CODEGEN_ASSERT!((free & mask) == 0)，但不依赖 CODEGEN_ASSERT 宏。
  assert!((*free & mask) == 0);

  *free |= mask;
}
