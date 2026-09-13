use core::mem::size_of;

use crate::{
  macros::codegen_assert::CODEGEN_ASSERT, records::block_iterator_wrapper::BlockIteratorWrapper,
};

impl BlockIteratorWrapper {
  pub fn operator_index(&self, pos: usize) -> u32 {
    CODEGEN_ASSERT!(
      pos < (self.it_end as usize).wrapping_sub(self.it_begin as usize) / size_of::<u32>()
    );
    unsafe { *self.it_begin.add(pos) }
  }
}
