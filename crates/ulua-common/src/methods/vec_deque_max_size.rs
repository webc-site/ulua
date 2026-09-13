use core::mem;

use crate::records::vec_deque::VecDeque;

impl<T> VecDeque<T> {
  pub fn max_size(&self) -> usize {
    usize::MAX / mem::size_of::<T>()
  }
}
