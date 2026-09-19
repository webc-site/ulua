use core::ptr::write;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::typed_allocator::TypedAllocator;
impl<T> TypedAllocator<T> {
  pub fn allocate(&mut self, value: T) -> *mut T {
    LUAU_ASSERT!(!self.frozen);

    if self.current_block_size >= Self::K_BLOCK_SIZE {
      LUAU_ASSERT!(self.current_block_size == Self::K_BLOCK_SIZE);
      self.append_block();
    }

    let block = *self.stuff.last().unwrap();
    let res = unsafe { block.add(self.current_block_size) };
    unsafe {
      write(res, value);
    }
    self.current_block_size += 1;
    res
  }
}
