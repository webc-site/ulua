use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper {
  pub fn begin(&self) -> *const u32 {
    self.it_begin
  }
}
