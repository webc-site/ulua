use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper {
  pub fn empty(&self) -> bool {
    self.it_begin == self.it_end
  }
}
