use crate::records::{ir_reg_alloc_x_64::IrRegAllocX64, scoped_spills::ScopedSpills};

impl ScopedSpills {
  pub fn scoped_spills_ir_reg_alloc_x_64(&mut self, owner: &mut IrRegAllocX64) {
    self.owner = owner as *mut IrRegAllocX64;
    self.start_spill_id = owner.next_spill_id;
  }
}
