use core::mem::size_of;

use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper {
  pub fn size(&self) -> usize {
    (self.it_end as usize).wrapping_sub(self.it_begin as usize) / size_of::<u32>()
  }
}
