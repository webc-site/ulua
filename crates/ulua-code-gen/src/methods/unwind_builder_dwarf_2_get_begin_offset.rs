use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;

impl UnwindBuilderDwarf2 {
  pub fn get_begin_offset(&self) -> usize {
    self.begin_offset
  }
}
