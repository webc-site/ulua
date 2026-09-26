use crate::type_aliases::instruction::Instruction;

/// # Safety
///
/// `pc` 必须指向当前执行 proto `code` 数组内、可写且按 `Instruction`(u32) 对齐的指令槽
/// （VM 反馈回填路径），写入即覆盖该槽整条指令字。cpp `lvmexecute.cpp:93` `VM_PATCH_AUX`
/// 同款宏，展开点须处于 VM 写反馈路径。
#[inline(always)]
pub unsafe fn vm_patch_aux(pc: *const Instruction, slot: i32) {
  // Safety: 契约保证 `pc` 为可写对齐指令槽，转 *mut 仅回写这 4 字节
  unsafe {
    *(pc as *mut Instruction) = slot as u32;
  }
}
