use crate::records::{
  assembly_builder_x_64::AssemblyBuilderX64, ir_lowering_x_64::IrLoweringX64,
  operand_x_64::OperandX64, register_x_64::RegisterX64,
};

impl IrLoweringX64 {
  #[inline]
  pub fn vector_and_mask_op(&mut self) -> OperandX64 {
    // cpp IrLoweringX64.cpp:4203 `if (vectorAndMask.base == noreg)`：
    // 初值为 noreg（Rust 端 NOREG，bits=0x80），而非 0xFF
    if self.vector_and_mask.base == RegisterX64::NOREG {
      self.vector_and_mask =
        AssemblyBuilderX64::u32x4(unsafe { &mut *self.build }, !0u32, !0u32, !0u32, 0u32);
    }

    self.vector_and_mask
  }
}
