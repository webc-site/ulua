use crate::type_aliases::instruction::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[inline(always)]
pub unsafe fn vm_patch_aux_slot(pc: *const Instruction, k: u32, slot: i32) {
  unsafe {
    *(pc as *mut Instruction) = k | ((slot as u32) << 16);
  }
}

pub use vm_patch_aux_slot as VM_PATCH_AUX_SLOT;
