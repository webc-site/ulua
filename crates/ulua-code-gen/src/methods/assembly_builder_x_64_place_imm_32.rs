use core::{mem::size_of_val, ptr::copy_nonoverlapping};

use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
  pub fn place_imm_32(&mut self, imm: i32) {
    let pos = self.code_pos;
    if !((pos as usize).wrapping_add(size_of_val(&imm)) < self.code_end as usize) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    unsafe {
      copy_nonoverlapping(imm.to_le_bytes().as_ptr(), pos, size_of_val(&imm));
      self.code_pos = pos.add(size_of_val(&imm));
    }
  }
}
