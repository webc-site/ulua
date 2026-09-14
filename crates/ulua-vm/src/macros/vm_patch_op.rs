use crate::type_aliases::instruction::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn vm_patch_op(pc: *const Instruction, op: u8) {
  unsafe {
    *(pc as *mut Instruction) = (op as u32) | (0xffffff00u32 & *pc);
  }
}

pub use vm_patch_op as VM_PATCH_OP;
