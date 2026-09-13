use crate::type_aliases::instruction::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn vm_patch_c(pc: *const Instruction, slot: i32) {
  unsafe {
    *(pc as *mut Instruction) = (((slot as u8) as u32) << 24) | (0x00ffffffu32 & *pc);
  }
}

pub use vm_patch_c as VM_PATCH_C;
