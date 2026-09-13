use crate::records::vec_deque::VecDeque;

impl<T> VecDeque<T> {
  pub(crate) fn logical_to_physical(&self, pos: usize) -> usize {
    let cap = self.capacity();
    if cap == 0 { 0 } else { (self.head + pos) % cap }
  }
}
