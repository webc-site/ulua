use crate::{macros::luau_assert::LUAU_ASSERT, records::vec_deque::VecDeque};

impl<T> VecDeque<T> {
  pub fn operator_index_mut(&mut self, pos: usize) -> &mut T {
    LUAU_ASSERT!(pos < self.queue_size);

    // SAFETY：LUAU_ASSERT 保证 pos < queue_size，物理槽位存有已初始化元素。
    unsafe { &mut *self.slot_ptr(pos) }
  }
}
