use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper {
  pub fn end(&self) -> *const u32 {
    self.it_end
  }
}
