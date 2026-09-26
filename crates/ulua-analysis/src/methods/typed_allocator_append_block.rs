use ulua_common::fflag;

use crate::{functions::paged_allocate::paged_allocate, records::typed_allocator::TypedAllocator};
impl<T> TypedAllocator<T> {
  pub(crate) fn append_block(&mut self) {
    // Commit to an allocation strategy on the first block and keep it for the
    // allocator's whole lifetime, so `free` deallocates the way it allocated
    // even if the (ScopedFastFlag) DebugLuauFreezeArena flag is toggled in
    // between. Reading the flag per-call mismatched VirtualFree/operator-delete
    // and corrupted the heap on Windows.
    if self.stuff.is_empty() {
      self.paged = fflag::DebugLuauFreezeArena.get();
    }
    // `None` 即分配失败（原 cpp `operator new(nothrow)` 返回 nullptr），按
    // bad_alloc 语义 panic；成功块折回裸指针入 arena（`stuff` 布局契约不变）。
    let Some(block) = paged_allocate(Self::K_BLOCK_SIZE_BYTES, self.paged) else {
      panic!("std::bad_alloc");
    };

    self.stuff.push(block.as_ptr().cast::<T>());
    self.current_block_size = 0;
  }
}
