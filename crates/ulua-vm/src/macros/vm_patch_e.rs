use crate::type_aliases::instruction::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn vm_patch_e(pc: *const Instruction, slot: i32) {
  unsafe {
    *(pc as *mut Instruction) = ((slot as u32) << 8) | (0x000000ffu32 & *pc);
  }
}

pub use vm_patch_e as VM_PATCH_E;
