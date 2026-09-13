use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
impl UnwindBuilderDwarf2 {
  pub fn set_begin_offset(&mut self, begin_offset: usize) {
    self.begin_offset = begin_offset;
  }
}
