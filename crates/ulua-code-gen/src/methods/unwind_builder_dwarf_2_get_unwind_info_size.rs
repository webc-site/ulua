use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;

impl UnwindBuilderDwarf2 {
  pub fn get_unwind_info_size(&self, _block_size: usize) -> usize {
    (unsafe { self.pos.offset_from(self.raw_data.as_ptr()) }) as usize
  }
}
