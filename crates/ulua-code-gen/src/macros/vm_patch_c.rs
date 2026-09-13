use crate::type_aliases::instruction_ir_translation::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn vm_patch_c(pc: *const Instruction, slot: i32) {
  unsafe {
    *(pc as *mut Instruction) = ((slot as u8 as u32) << 24) | (0x00ffffffu32 & *pc);
  }
}
