use core::{mem::size_of, ptr::from_ref};

use crate::records::{ir_function::IrFunction, ir_inst::IrInst};

macro_rules! CODEGEN_ASSERT {
  ($expr:expr) => {
    assert!($expr);
  };
}

impl IrFunction {
  pub fn get_inst_index(&self, inst: &IrInst) -> u32 {
    // 只能传入本 vector 中的指令
    let inst_ptr = from_ref(inst).cast::<u8>() as usize;
    let base_ptr = self.instructions.as_ptr().cast::<u8>() as usize;
    let end_ptr = base_ptr + self.instructions.len() * size_of::<IrInst>();

    CODEGEN_ASSERT!(inst_ptr >= base_ptr && inst_ptr <= end_ptr);

    let offset = inst_ptr - base_ptr;
    (offset / size_of::<IrInst>()) as u32
  }
}
