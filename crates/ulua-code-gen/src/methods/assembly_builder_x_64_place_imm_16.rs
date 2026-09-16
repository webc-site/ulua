use core::mem::size_of;

use crate::{
  functions::writeu_16::writeu_16, macros::codegen_assert::CODEGEN_ASSERT,
  records::assembly_builder_x_64::AssemblyBuilderX64,
};

impl AssemblyBuilderX64 {
  pub fn place_imm_16(&mut self, imm: i16) {
    let pos = self.code_pos;
    unsafe {
      CODEGEN_ASSERT!(pos.add(size_of::<i16>()) < self.code_end);
      self.code_pos = writeu_16(pos, imm as u16);
    }
  }
}
