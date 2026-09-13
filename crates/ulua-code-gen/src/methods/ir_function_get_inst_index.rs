use core::mem::size_of;

use crate::records::{ir_function::IrFunction, ir_inst::IrInst};

macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

impl IrFunction {
  pub fn get_inst_index(&self, inst: &IrInst) -> u32 {
    // Can only be called with instructions from our vector
    let inst_ptr = inst as *const IrInst as usize;
    let base_ptr = self.instructions.as_ptr() as usize;
    let end_ptr = unsafe { self.instructions.as_ptr().add(self.instructions.len()) } as usize;

    CODEGEN_ASSERT!(inst_ptr >= base_ptr && inst_ptr <= end_ptr);

    let offset = inst_ptr - base_ptr;
    (offset / size_of::<IrInst>()) as u32
  }
}
