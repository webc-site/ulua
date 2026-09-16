use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_reg_alloc_a_64::IrRegAllocA64, register_a_64::RegisterA64},
};

impl IrRegAllocA64 {
  pub fn take_reg(&mut self, reg: RegisterA64, index: u32) -> RegisterA64 {
    let set = self.get_set(reg.kind());

    CODEGEN_ASSERT!((set.free & (1u32 << reg.index())) != 0);
    CODEGEN_ASSERT!(set.defs[reg.index() as usize] == IrRegAllocA64::K_INVALID_INST_IDX);

    set.free &= !(1u32 << reg.index());
    set.defs[reg.index() as usize] = index;

    reg
  }
}
