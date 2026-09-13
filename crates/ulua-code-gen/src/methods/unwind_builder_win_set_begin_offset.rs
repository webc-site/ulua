use crate::records::unwind_builder_win::UnwindBuilderWin;

impl UnwindBuilderWin {
  pub fn set_begin_offset(&mut self, begin_offset: usize) {
    self.begin_offset = begin_offset;
  }
}
