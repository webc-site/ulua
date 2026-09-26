use crate::{
  enums::kind_a_64::KindA64,
  functions::countrz_bit_utils::countrz_u64,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::emit_common_a_64::{K_SPILL_SLOTS, K_STACK_SIZE},
};

pub fn alloc_spill(free: &mut u64, kind: KindA64) -> i32 {
  // 为支持更大的栈帧，qN 必须分配在 16b 边界上，才能塞进 ldr/str 编码
  // cpp IrRegAllocA64.cpp:28 `CODEGEN_ASSERT(kStackSize <= 256);` —— 帧尺寸由单一常量源保障
  CODEGEN_ASSERT!(K_STACK_SIZE <= 256);

  let mut search = *free;

  // qN 寄存器占用两个相邻槽位
  if kind == KindA64::Q {
    // 确保 bit N 置位时 bit N+1 也置位
    search = *free & (*free >> 1);

    // 禁止 qN 分配在栈/额外 spill 存储的边界处（预留最后一个栈槽）
    // cpp IrRegAllocA64.cpp:39 `search &= ~(1ull << (kSpillSlots - 1));`，边界与 is_extra_spill_slot
    // 共用 emit_common_a_64::K_SPILL_SLOTS = 22。
    search &= !(1u64 << (K_SPILL_SLOTS - 1));
  }

  let slot = countrz_u64(search);
  if slot == 64 {
    return -1;
  }

  let mask = (if kind == KindA64::Q { 3u64 } else { 1u64 }) << (slot as u64);

  CODEGEN_ASSERT!((*free & mask) == mask);

  *free &= !mask;

  slot
}
