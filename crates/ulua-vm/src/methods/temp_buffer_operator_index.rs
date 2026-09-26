use core::ops::{Index, IndexMut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::temp_buffer::TempBuffer;

// cpp `TempBuffer::operator[]` 的 Rust 形态即 Index/IndexMut：不另设同名方法，
// 调用点一律走 `buf[i]` 下标语法。
impl<T> Index<usize> for TempBuffer<T> {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    LUAU_ASSERT!(index < self.count);
    // Safety: 不变式（allocate 契约建立、本缓冲唯一写点）保证 data 指向 count 个已初始化 T，
    // 且 index < count（release 下 LUAU_ASSERT 虽可编译掉，破坏该不变式即为调用方违约）
    unsafe { &*self.data.add(index) }
  }
}

impl<T> IndexMut<usize> for TempBuffer<T> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    LUAU_ASSERT!(index < self.count);
    // Safety: 同上不变式——data 指向 count 个已初始化 T，index < count 时该槽独占可变借用成立
    unsafe { &mut *self.data.add(index) }
  }
}
