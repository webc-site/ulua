use crate::records::{ir_reg_alloc_x_64::IrRegAllocX64, register_x_64::RegisterX64};

#[derive(Debug)]
#[repr(C)]
pub struct ScopedRegX64 {
  pub owner: *mut IrRegAllocX64,
  pub reg: RegisterX64,
}
