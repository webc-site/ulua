use crate::type_aliases::instruction::Instruction;

/// # Safety
///
/// `pc` 必须指向当前执行 proto `code` 数组内、可写且按 `Instruction`(u32) 对齐的指令槽
/// （反馈回填 aux：低 16 位 = `k`、高 16 位 = `slot`）。cpp `lvmexecute.cpp:94`
/// `VM_PATCH_AUX_SLOT` 同款宏，展开点均以 `pc-2` 回滚定位，须处于 VM 写反馈路径。
#[inline(always)]
pub unsafe fn vm_patch_aux_slot(pc: *const Instruction, k: u32, slot: i32) {
  // Safety: 契约保证 `pc` 为可写对齐指令槽，读改写仅触及这 4 字节
  unsafe {
    *(pc as *mut Instruction) = k | ((slot as u32) << 16);
  }
}
