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

/// # Safety
///
/// `stuff: Vec<*mut T>` 是本分配器独占拥有的堆块，令自动 Send 失效。当 `T: Send`
/// 时，转移守卫即转移块及其 `T` 内容的唯一所有权（与原 `Vec<*mut T>` 所有者相同的
/// 独占性），无并发访问者，故 Send 可靠。
// Safety: stuff 是 Vec<*mut T>（自动 Send 因裸指针失效）。裸指针指向本分配器
// 独占的堆块，转移 TypedAllocator 即转移这些块唯一且完整的所有权（Default/构造
// 点不与他者共享），T: Send 保证块内容本身可移动，转移后原线程不再持有任何句柄。
unsafe impl<T: Send> Send for TypedAllocator<T> {}
/// # Safety
///
/// 当 `T: Sync` 时，共享 `&TypedAllocator<T>` 仅经由 `stuff` 以 `&T` 等价的只读
/// 方式访问已分配的 `T`（追加/冻结需 `&mut`），故并发只读共享不产生数据竞争。
// Safety: 共享 &TypedAllocator<T> 时唯一可达的 T 数据是 stuff 里的块内容；追加、
// freeze/unfreeze 均须 &mut self（编译期排斥并发写），只读路径不产生 &mut，故
// T: Sync 时并发持有 &T 是纯读，满足 Sync 契约。
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
