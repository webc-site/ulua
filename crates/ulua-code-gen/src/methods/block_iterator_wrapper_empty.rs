use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper<'_> {
  pub fn empty(&self) -> bool {
    self.as_slice().is_empty()
  }
}
