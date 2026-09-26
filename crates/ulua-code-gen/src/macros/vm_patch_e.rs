use crate::type_aliases::instruction_ir_builder::Instruction;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn vm_patch_e(pc: *mut Instruction, slot: i32) {
  // Safety: 契约保证 pc 指向字节码缓冲内存活可写的合法 Instruction 槽；读 *pc 做位掩码
  // 后写回同址，读写类型一致（Instruction/u32 对齐相同），单线程串行 patch 无别名冲突。
  unsafe {
    *pc = ((slot as u32) << 8) | (0x000000ffu32 & *pc);
  }
}
