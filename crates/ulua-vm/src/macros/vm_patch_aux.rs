use crate::type_aliases::instruction::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn vm_patch_aux(pc: *const Instruction, slot: i32) {
  unsafe {
    *(pc as *mut Instruction) = slot as u32;
  }
}

pub use vm_patch_aux as VM_PATCH_AUX;
