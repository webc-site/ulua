use core::{ffi::c_void, ptr::drop_in_place};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::paged_deallocate::paged_deallocate, records::typed_allocator::TypedAllocator,
};
impl<T> TypedAllocator<T> {
  pub(crate) fn free(&mut self) {
    LUAU_ASSERT!(!self.frozen);

    let last_block = self.stuff.last().copied();

    for &block in &self.stuff {
      let block_size = if Some(block) == last_block {
        self.current_block_size
      } else {
        Self::K_BLOCK_SIZE
      };

      for i in 0..block_size {
        unsafe {
          drop_in_place(block.add(i));
        }
      }

      paged_deallocate(block as *mut c_void, Self::K_BLOCK_SIZE_BYTES, self.paged);
    }

    self.stuff.clear();
    self.current_block_size = 0;
  }
}
