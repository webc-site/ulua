use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper<'_> {
  pub fn operator_index(&self, pos: usize) -> u32 {
    self.as_slice()[pos]
  }
}
