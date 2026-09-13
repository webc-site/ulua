use core::mem::size_of;

use crate::{
  functions::writeu_64::writeu_64, macros::codegen_assert::CODEGEN_ASSERT,
  records::assembly_builder_x_64::AssemblyBuilderX64,
};

impl AssemblyBuilderX64 {
  pub fn place_imm_64(&mut self, imm: i64) {
    let pos = self.code_pos;
    unsafe {
      CODEGEN_ASSERT!(pos.add(size_of::<i64>()) < self.code_end);
      self.code_pos = writeu_64(pos, imm as u64);
    }
  }
}
