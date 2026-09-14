use crate::records::{ir_reg_alloc_a_64::IrRegAllocA64, register_a_64::RegisterA64};

macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

impl IrRegAllocA64 {
  pub fn free_temp(&mut self, reg: RegisterA64) {
    let kind = reg.kind();
    let index = reg.index();
    let bit = 1u32 << index;

    let set = self.get_set(kind);

    CODEGEN_ASSERT!((set.base & bit) != 0);
    CODEGEN_ASSERT!((set.free & bit) == 0);
    CODEGEN_ASSERT!((set.temp & bit) != 0);

    set.free |= bit;
    set.temp &= !bit;
  }
}
