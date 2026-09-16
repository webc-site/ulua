use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_reg_alloc_a_64::IrRegAllocA64, register_a_64::RegisterA64},
};

impl IrRegAllocA64 {
  pub fn free_reg(&mut self, reg: RegisterA64) {
    let set = self.get_set(reg.kind());

    let bit = 1u32 << reg.index();

    CODEGEN_ASSERT!((set.base & bit) != 0);
    CODEGEN_ASSERT!((set.free & bit) == 0);
    CODEGEN_ASSERT!((set.temp & bit) == 0);

    set.free |= bit;
    set.defs[reg.index() as usize] = Self::K_INVALID_INST_IDX;
  }
}
