use crate::type_aliases::instruction::Instruction;

/// # Safety
///
/// `pc` 必须指向当前执行 proto `code` 数组内、可读且可写、按 `Instruction`(u32) 对齐的
/// 指令槽（仅替换 C 字段 bits24..31，保留低 24 位）。cpp `lvmexecute.cpp:91`
/// `VM_PATCH_C` 同款宏，展开点（如 cachedslot 回填）须处于 VM 写反馈路径。
#[inline(always)]
pub unsafe fn vm_patch_c(pc: *const Instruction, slot: i32) {
  // Safety: 契约保证 `pc` 可读写对齐，先读低 24 位再回写同槽
  unsafe {
    *(pc as *mut Instruction) = (((slot as u8) as u32) << 24) | (0x00ffffffu32 & *pc);
  }
}
