use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper<'_> {
  pub fn size(&self) -> usize {
    self.len()
  }
}
