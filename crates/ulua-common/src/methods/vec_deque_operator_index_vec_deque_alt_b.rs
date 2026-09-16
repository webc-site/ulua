use crate::{macros::luau_assert::LUAU_ASSERT, records::vec_deque::VecDeque};

impl<T> VecDeque<T> {
  #[inline]
  pub fn operator_index(&self, pos: usize) -> &T {
    LUAU_ASSERT!(pos < self.queue_size);

    // SAFETY：LUAU_ASSERT 保证 pos < queue_size，物理槽位存有已初始化元素。
    unsafe { &*self.slot_ptr(pos) }
  }
}
