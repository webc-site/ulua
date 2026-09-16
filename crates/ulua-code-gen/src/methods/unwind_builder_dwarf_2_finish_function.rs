use crate::{
  functions::{align_position::align_position, writeu_32::writeu_32},
  records::unwind_builder_dwarf_2::UnwindBuilderDwarf2,
};

impl UnwindBuilderDwarf2 {
  pub fn finish_function(&mut self, begin_offset: u32, end_offset: u32) {
    if let Some(last_func) = self.unwind_functions.last_mut() {
      last_func.begin_offset = begin_offset;
      last_func.end_offset = end_offset;
    }

    ulua_common::LUAU_ASSERT!(!self.fde_entry_start.is_null());

    unsafe {
      self.pos = align_position(self.fde_entry_start, self.pos);
      let length = (self.pos as usize - self.fde_entry_start as usize - 4) as u32;
      writeu_32(self.fde_entry_start, length);
    }
  }
}
