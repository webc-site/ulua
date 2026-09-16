use core::{ffi::c_void, ptr::drop_in_place, slice};

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

      // 单次建切片迭代析构，免逐索引 add 与越界检查
      for elem in unsafe { slice::from_raw_parts_mut(block, block_size) } {
        // SAFETY：切片覆盖整块存活元素，与上游逐下标析构等价
        unsafe { drop_in_place(elem as *mut T) };
      }

      paged_deallocate(block as *mut c_void, Self::K_BLOCK_SIZE_BYTES, self.paged);
    }

    self.stuff.clear();
    self.current_block_size = 0;
  }
}
