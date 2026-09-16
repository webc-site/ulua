use core::mem;

use crate::records::vec_deque::VecDeque;

impl<T> VecDeque<T> {
  pub fn max_size(&self) -> usize {
    // ZST 无存储开销，容量仅受 usize 索引域限制（C++ 的除法对 ZST 是除零
    // UB，此处收敛为饱和值而非 panic）。
    usize::MAX
      .checked_div(mem::size_of::<T>())
      .unwrap_or(usize::MAX)
  }
}
