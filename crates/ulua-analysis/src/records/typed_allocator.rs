use alloc::vec::Vec;
use core::mem::size_of;
#[derive(Debug)]
pub struct TypedAllocator<T> {
  pub(crate) frozen: bool,
  pub(crate) stuff: Vec<*mut T>,
  pub(crate) current_block_size: usize,
  /// The allocation strategy this allocator committed to, captured from
  /// `DebugLuauFreezeArena` at the FIRST block allocation. Every block is both
  /// allocated and freed with this same value, so a later toggle of the
  /// (ScopedFastFlag) global flag can't mismatch VirtualFree/operator-delete and
  /// corrupt the heap. Meaningless while `stuff` is empty.
  pub(crate) paged: bool,
}

impl<T> TypedAllocator<T> {
  pub(crate) const K_BLOCK_SIZE_BYTES: usize = 32768;
  pub(crate) const K_BLOCK_SIZE: usize = Self::K_BLOCK_SIZE_BYTES / size_of::<T>();
}

unsafe impl<T: Send> Send for TypedAllocator<T> {}
unsafe impl<T: Sync> Sync for TypedAllocator<T> {}

impl<T> Default for TypedAllocator<T> {
  fn default() -> Self {
    Self {
      frozen: false,
      stuff: Vec::new(),
      current_block_size: Self::K_BLOCK_SIZE,
      paged: false,
    }
  }
}
