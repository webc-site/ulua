use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;

#[derive(Debug)]
#[repr(C)]
pub struct ScopedSpills {
  pub(crate) owner: *mut IrRegAllocX64,
  pub(crate) start_spill_id: u32,
}
