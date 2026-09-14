use core::{cmp, ptr};

use crate::records::vec_deque::VecDeque;

impl<T> VecDeque<T> {
  pub(crate) fn destroy_elements(&mut self) {
    if let Some(buf) = self.buffer {
      let head_size = cmp::min(self.queue_size, self.capacity().saturating_sub(self.head));
      let tail_size = self.queue_size - head_size;

      // SAFETY：head/tail 两段切片均落在缓冲区容量内且元素已初始化，
      // drop_in_place 按顺序逐个析构（与 C++ 的两个析构循环等价）。
      unsafe {
        ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
          buf.as_ptr().add(self.head),
          head_size,
        ));
        ptr::drop_in_place(ptr::slice_from_raw_parts_mut(buf.as_ptr(), tail_size));
      }
    }
  }
}
