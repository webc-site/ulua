use crate::type_aliases::instruction::Instruction;

/// # Safety
///
/// `pc` 必须指向当前执行 proto `code` 数组内、可读且可写、按 `Instruction`(u32) 对齐的
/// 指令槽（仅替换操作码字节 bits0..8，保留其余位）。cpp `lvmexecute.cpp:90`
/// `VM_PATCH_OP` 同款宏，展开点须处于 VM 写反馈路径。
#[inline(always)]
pub unsafe fn vm_patch_op(pc: *const Instruction, op: u8) {
  // Safety: 契约保证 `pc` 可读写对齐，先读保留位再回写同槽
  unsafe {
    *(pc as *mut Instruction) = (op as u32) | (0xffffff00u32 & *pc);
  }
}
