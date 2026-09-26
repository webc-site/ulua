use crate::type_aliases::instruction_ir_builder::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn vm_patch_c(pc: *const Instruction, slot: i32) {
  // Safety: 契约保证 pc 指向字节码缓冲内存活的合法 Instruction 槽，转 *mut 仅就地
  // 改写当前槽；读写同为 Instruction(u32) 对齐一致，单线程串行 patch 无别名冲突。
  unsafe {
    *(pc as *mut Instruction) = ((slot as u8 as u32) << 24) | (0x00ffffffu32 & *pc);
  }
}
