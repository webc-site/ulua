use alloc::alloc::{alloc, dealloc};
use core::{
  alloc::Layout,
  cmp,
  fmt::{Debug, Formatter, Result as FmtResult},
  iter::{Chain, ExactSizeIterator, FusedIterator},
  marker::PhantomData,
  ptr::{self, NonNull},
  slice,
};

/// C++ 增长因子 1.5x（folly FBVector 风格，便于内存复用、减少碎片）。
const GROWTH_FACTOR_NUM: usize = 3;
const GROWTH_FACTOR_DEN: usize = 2;
/// 空容器首次增长的容量（C++ grow 的常数项）。
const INITIAL_CAPACITY: usize = 4;

pub struct VecDeque<T> {
  pub(crate) buffer: Option<NonNull<T>>,
  pub(crate) buffer_capacity: usize,
  pub(crate) head: usize,
  pub(crate) queue_size: usize,
  pub(crate) _marker: PhantomData<T>,
}

impl<T> VecDeque<T> {
  /// `VecDeque(std::initializer_list<T>)` — construct from a list, reserving
  /// exactly the element count so the buffer is contiguous with
  /// `capacity == size` (no growth), matching the C++ initializer-list ctor.
  pub fn from_init_list(items: Vec<T>) -> Self {
    let mut q = Self::new();
    if !items.is_empty() {
      q.reserve(items.len());
      for item in items {
        q.push_back(item);
      }
    }
    q
  }

  pub(crate) fn allocate(&self, capacity: usize) -> NonNull<T> {
    if capacity == 0 {
      panic!("Zero capacity allocation");
    }
    let layout = Layout::array::<T>(capacity).unwrap();
    // ZST：`allocate(n * 0)` 的零大小 layout 被 `alloc` 禁止（C++
    // `allocator<T>::allocate` 对 ZST 合法）；用对齐的悬垂指针承载，
    // ZST 读写均为 no-op，逻辑槽位照常成立。
    if layout.size() == 0 {
      return NonNull::dangling();
    }
    unsafe {
      let ptr = alloc(layout);
      NonNull::new(ptr as *mut T).expect("Allocation failed")
    }
  }

  pub(crate) fn deallocate(&self, ptr: Option<NonNull<T>>, capacity: usize) {
    if let Some(p) = ptr
      && capacity > 0
    {
      let layout = Layout::array::<T>(capacity).unwrap();
      // ZST 块是悬垂指针，未真正分配，跳过释放。
      if layout.size() != 0 {
        unsafe {
          dealloc(p.as_ptr() as *mut u8, layout);
        }
      }
    }
  }

  /// C++ 增长策略：`1.5x + 1`，空容器取 [`INITIAL_CAPACITY`]。
  fn grow_capacity(&self) -> usize {
    let old_capacity = self.capacity();
    if old_capacity == 0 {
      INITIAL_CAPACITY
    } else {
      old_capacity * GROWTH_FACTOR_NUM / GROWTH_FACTOR_DEN + 1
    }
  }

  /// 分配 `new_capacity` 的新缓冲区，按位搬迁全部元素（C++
  /// `uninitialized_move`），释放旧缓冲区，`head` 归零。
  ///
  /// 搬迁是按位拷贝且旧缓冲区不做 drop（等价于移动语义，元素由新缓冲区独占）。
  fn reallocate(&mut self, new_capacity: usize) {
    let old_capacity = self.buffer_capacity;
    let head_size = cmp::min(self.queue_size, old_capacity - self.head);
    let tail_size = self.queue_size - head_size;
    let new_buffer = self.allocate(new_capacity);

    // SAFETY：head/tail 两段索引均落在旧缓冲区容量内；新缓冲区是刚分配的
    // 未初始化内存，与旧缓冲区不重叠，且容量足以容纳全部元素。
    unsafe {
      if let Some(old_buf) = self.buffer {
        if head_size != 0 {
          ptr::copy_nonoverlapping(
            old_buf.as_ptr().add(self.head),
            new_buffer.as_ptr(),
            head_size,
          );
        }
        if tail_size != 0 {
          ptr::copy_nonoverlapping(
            old_buf.as_ptr(),
            new_buffer.as_ptr().add(head_size),
            tail_size,
          );
        }
      }
    }

    self.deallocate(self.buffer, old_capacity);

    self.buffer = Some(new_buffer);
    self.buffer_capacity = new_capacity;
    self.head = 0;
  }

  pub(crate) fn grow(&mut self) {
    let new_capacity = self.grow_capacity();
    if new_capacity > self.max_size() {
      panic!("bad_array_new_length");
    }
    self.reallocate(new_capacity);
  }

  /// 逻辑位置 `pos` 的物理槽位指针。
  ///
  /// # Safety
  /// 调用方须保证容器已分配缓冲区，且该逻辑槽位当前存有元素（或 push 路径
  /// 已保证容量），否则写入未初始化内存 / 读到无效元素。
  pub(crate) unsafe fn slot_ptr(&self, pos: usize) -> *mut T {
    // SAFETY：容器不变量保证 head < capacity，逻辑位置经取模后落在容量内。
    unsafe {
      self
        .buffer
        .unwrap()
        .as_ptr()
        .add(self.logical_to_physical(pos))
    }
  }

  /// 逻辑顺序（front → back）的两个连续切片；无环绕时第二段为空。
  fn slices(&self) -> (&[T], &[T]) {
    if self.queue_size == 0 {
      return (&[], &[]);
    }
    let head_size = cmp::min(self.queue_size, self.buffer_capacity - self.head);
    // SAFETY：head 段 [head, head+head_size) 与环绕段 [0, size-head_size)
    // 均落在容量范围内，槽位存有已初始化元素；非空保证 buffer 已分配。
    unsafe {
      let buf = self.buffer.unwrap().as_ptr();
      (
        slice::from_raw_parts(buf.add(self.head), head_size),
        slice::from_raw_parts(buf, self.queue_size - head_size),
      )
    }
  }

  /// [`VecDeque::slices`] 的可变版本。
  fn slices_mut(&mut self) -> (&mut [T], &mut [T]) {
    if self.queue_size == 0 {
      return (&mut [], &mut []);
    }
    let head_size = cmp::min(self.queue_size, self.buffer_capacity - self.head);
    // SAFETY：与 `slices` 相同；`&mut self` 保证唯一访问。
    unsafe {
      let buf = self.buffer.unwrap().as_ptr();
      (
        slice::from_raw_parts_mut(buf.add(self.head), head_size),
        slice::from_raw_parts_mut(buf, self.queue_size - head_size),
      )
    }
  }

  pub fn push_back(&mut self, value: T) {
    if self.is_full() {
      self.grow();
    }
    // SAFETY：is_full 时已增长，queue_size 槽位可写。
    unsafe { ptr::write(self.slot_ptr(self.queue_size), value) };
    self.queue_size += 1;
  }

  pub fn pop_back(&mut self) {
    assert!(!self.empty());
    self.queue_size -= 1;
    // SAFETY：原 queue_size > 0 保证该物理槽位存有已初始化元素。
    unsafe { ptr::drop_in_place(self.slot_ptr(self.queue_size)) };
  }

  pub fn push_front(&mut self, value: T) {
    if self.is_full() {
      self.grow();
    }
    self.head = if self.head == 0 {
      self.capacity() - 1
    } else {
      self.head - 1
    };
    // SAFETY：is_full 时已增长，head 槽位可写。
    unsafe { ptr::write(self.slot_ptr(0), value) };
    self.queue_size += 1;
  }

  pub fn pop_front(&mut self) {
    assert!(!self.empty());
    // SAFETY：非空保证 head 槽位存有已初始化元素。
    unsafe { ptr::drop_in_place(self.slot_ptr(0)) };
    self.head += 1;
    self.queue_size -= 1;
    if self.head == self.capacity() {
      self.head = 0;
    }
  }

  pub fn clear(&mut self) {
    self.destroy_elements();
    self.head = 0;
    self.queue_size = 0;
  }

  pub fn reserve(&mut self, new_capacity: usize) {
    if new_capacity > self.max_size() {
      panic!("too large");
    }
    if new_capacity <= self.capacity() {
      return;
    }
    self.reallocate(new_capacity);
  }

  pub fn shrink_to_fit(&mut self) {
    if self.capacity() == self.queue_size {
      return;
    }
    if self.queue_size == 0 {
      self.destroy_elements();
      self.deallocate(self.buffer, self.buffer_capacity);
      self.buffer = None;
      self.buffer_capacity = 0;
      self.head = 0;
      return;
    }
    self.reallocate(self.queue_size);
  }

  pub fn at(&self, pos: usize) -> &T {
    if pos >= self.queue_size {
      panic!("VecDeque out of range");
    }
    // SAFETY：pos < queue_size 保证该物理槽位存有已初始化元素。
    unsafe { &*self.slot_ptr(pos) }
  }

  pub fn at_mut(&mut self, pos: usize) -> &mut T {
    if pos >= self.queue_size {
      panic!("VecDeque out of range");
    }
    // SAFETY：pos < queue_size 保证该物理槽位存有已初始化元素。
    unsafe { &mut *self.slot_ptr(pos) }
  }

  pub fn front(&self) -> &T {
    assert!(!self.empty());
    // SAFETY：非空保证 head 槽位存有已初始化元素。
    unsafe { &*self.slot_ptr(0) }
  }

  pub fn front_mut(&mut self) -> &mut T {
    assert!(!self.empty());
    // SAFETY：非空保证 head 槽位存有已初始化元素。
    unsafe { &mut *self.slot_ptr(0) }
  }

  pub fn back(&self) -> &T {
    assert!(!self.empty());
    // SAFETY：非空保证 queue_size - 1 槽位存有已初始化元素。
    unsafe { &*self.slot_ptr(self.queue_size - 1) }
  }

  pub fn back_mut(&mut self) -> &mut T {
    assert!(!self.empty());
    // SAFETY：非空保证 queue_size - 1 槽位存有已初始化元素。
    unsafe { &mut *self.slot_ptr(self.queue_size - 1) }
  }

  /// 按逻辑顺序（front → back）迭代元素。环缓冲按
  /// `[head, capacity)` + `[0, 环绕部分)` 两段拼接，语义与
  /// `std::collections::VecDeque::iter` 一致。
  pub fn iter(&self) -> Iter<'_, T> {
    let (head, tail) = self.slices();
    Iter(head.iter().chain(tail.iter()))
  }

  /// [`VecDeque::iter`] 的可变版本。
  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    let (head, tail) = self.slices_mut();
    IterMut(head.iter_mut().chain(tail.iter_mut()))
  }
}

impl<T> Drop for VecDeque<T> {
  fn drop(&mut self) {
    self.destroy_elements();
    self.deallocate(self.buffer, self.buffer_capacity);
  }
}

impl<T: Debug> Debug for VecDeque<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "VecDeque")?;
    f.debug_list().entries(self.iter()).finish()
  }
}

impl<T: Clone> Clone for VecDeque<T> {
  fn clone(&self) -> Self {
    let mut new_deque = VecDeque::new();
    if self.buffer_capacity == 0 {
      return new_deque;
    }
    let new_buffer = new_deque.allocate(self.buffer_capacity);
    new_deque.buffer = Some(new_buffer);
    new_deque.buffer_capacity = self.buffer_capacity;
    new_deque.head = self.head;
    // queue_size 从 0 起逐元素推进：`val.clone()` panic 时 Drop 只析构已
    // 写入的槽位（head 段 [head, head+k) ∪ 环绕段 [0, j) 恰由
    // queue_size == k + j 表达），不会触碰未初始化内存。

    let (head_part, tail_part) = self.slices();
    for (i, val) in head_part.iter().enumerate() {
      let cloned = val.clone();
      // SAFETY：新缓冲区是刚分配的未初始化内存，容量与旧缓冲区相同，
      // 槽位一一对应，用 ptr::write 逐元素克隆写入。
      unsafe { ptr::write(new_buffer.as_ptr().add(self.head + i), cloned) };
      new_deque.queue_size += 1;
    }
    for (i, val) in tail_part.iter().enumerate() {
      let cloned = val.clone();
      // SAFETY：同上，环绕段写入物理槽 [0, tail)。
      unsafe { ptr::write(new_buffer.as_ptr().add(i), cloned) };
      new_deque.queue_size += 1;
    }
    new_deque
  }

  /// C++ 拷贝赋值 `operator=（const VecDeque&）`：容量足够时复用当前缓冲区
  /// （不足则按源容量重新分配），`head` 归零——赋值后的队列总是 contiguous，
  /// 与拷贝构造保留 `head` 的行为不同。Reference: `VecDeque.h:202-236`。
  fn clone_from(&mut self, other: &Self) {
    self.destroy_elements();
    // destroy_elements 只析构不重置计数；立刻归零，保证此后任意 panic
    // 时刻 Drop 看到的 queue_size 恰等于已写入元素数。
    self.queue_size = 0;
    if self.buffer_capacity < other.queue_size {
      self.deallocate(self.buffer, self.buffer_capacity);
      let new_buffer = self.allocate(other.buffer_capacity);
      self.buffer = Some(new_buffer);
      self.buffer_capacity = other.buffer_capacity;
    }
    self.head = 0;

    let Some(buf) = self.buffer else {
      return;
    };
    let (head_part, tail_part) = other.slices();
    for (i, val) in head_part.iter().enumerate() {
      let cloned = val.clone();
      // SAFETY：capacity ≥ queue_size（上面已按需重分配），head == 0，
      // 两段元素依次写入 [0, queue_size) 区间的未初始化内存。
      unsafe { ptr::write(buf.as_ptr().add(i), cloned) };
      self.queue_size += 1;
    }
    for (i, val) in tail_part.iter().enumerate() {
      let cloned = val.clone();
      // SAFETY：同上，环绕段续写在 head 段之后。
      unsafe { ptr::write(buf.as_ptr().add(head_part.len() + i), cloned) };
      self.queue_size += 1;
    }
  }
}

/// 前向迭代器：按逻辑顺序（front → back）产出元素。newtype 转发两段切片的
/// `Chain`，并手动补 `ExactSizeIterator`（std 未为 `Chain` 提供该实现）。
pub struct Iter<'a, T>(Chain<slice::Iter<'a, T>, slice::Iter<'a, T>>);

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<&'a T> {
    self.0.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.0.size_hint()
  }

  fn count(self) -> usize {
    self.0.count()
  }

  fn last(self) -> Option<&'a T> {
    self.0.last()
  }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
  fn next_back(&mut self) -> Option<&'a T> {
    self.0.next_back()
  }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> FusedIterator for Iter<'_, T> {}

/// [`Iter`] 的可变版本。
pub struct IterMut<'a, T>(Chain<slice::IterMut<'a, T>, slice::IterMut<'a, T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<&'a mut T> {
    self.0.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.0.size_hint()
  }

  fn count(self) -> usize {
    self.0.count()
  }

  fn last(self) -> Option<&'a mut T> {
    self.0.last()
  }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
  fn next_back(&mut self) -> Option<&'a mut T> {
    self.0.next_back()
  }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}
impl<T> FusedIterator for IterMut<'_, T> {}

/// 所有权迭代器：按逻辑顺序（front → back）消费元素；未消费的元素随
/// 迭代器一起析构，缓冲区最后释放。
pub struct IntoIter<T> {
  deque: VecDeque<T>,
}

impl<T> IntoIter<T> {
  /// 剩余元素个数。
  pub fn len(&self) -> usize {
    self.deque.queue_size
  }

  pub fn is_empty(&self) -> bool {
    self.deque.queue_size == 0
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<T> {
    if self.deque.queue_size == 0 {
      return None;
    }
    // SAFETY：queue_size > 0 保证 buffer 已分配且 head 槽位存有元素；
    // 读走后槽位逻辑移除，无需再 drop。
    let value = unsafe { ptr::read(self.deque.buffer.unwrap().as_ptr().add(self.deque.head)) };
    self.deque.head += 1;
    if self.deque.head == self.deque.buffer_capacity {
      self.deque.head = 0;
    }
    self.deque.queue_size -= 1;
    Some(value)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = self.deque.queue_size;
    (len, Some(len))
  }

  fn count(self) -> usize {
    self.deque.queue_size
  }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
  fn next_back(&mut self) -> Option<T> {
    let last = self.deque.queue_size.checked_sub(1)?;
    self.deque.queue_size = last;
    // SAFETY：queue_size > 0（原值）保证该物理槽位存有已初始化元素。
    unsafe { Some(ptr::read(self.deque.slot_ptr(last))) }
  }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}

impl<T> Drop for IntoIter<T> {
  fn drop(&mut self) {
    // 已读走的元素不在 queue_size 内；clear 只析构剩余元素。
    self.deque.clear();
  }
}

impl<'a, T> IntoIterator for &'a VecDeque<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Iter<'a, T> {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut VecDeque<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> IterMut<'a, T> {
    self.iter_mut()
  }
}

impl<T> IntoIterator for VecDeque<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;

  fn into_iter(self) -> IntoIter<T> {
    IntoIter { deque: self }
  }
}

#[cfg(test)]
mod tests {
  use alloc::vec::Vec;

  use super::VecDeque;

  /// ZST 元素的双端操作 + grow + 迭代：缓冲区用 dangling 指针承载
  /// （`alloc`/`dealloc` 禁止零大小 layout），`max_size` 不除零。
  #[test]
  fn zst_push_pop_grow_iter() {
    let mut q: VecDeque<u8> = VecDeque::new();
    assert_eq!(q.max_size(), usize::MAX);
    for _ in 0..50 {
      q.push_back(0);
      q.push_front(0);
    }
    assert_eq!(q.size(), 100);
    assert_eq!(q.iter().count(), 100);
    assert_eq!(q.into_iter().count(), 100);

    let mut r: VecDeque<u8> = VecDeque::new();
    r.push_back(0);
    r.pop_back();
    assert!(r.empty());
  }

  /// 环绕布局（head 在缓冲区中段、逻辑序列跨越物理末尾）下的克隆：拷贝
  /// 构造保留 head，`clone_from` 归零（contiguous）。回归覆盖 Clone 写入
  /// head 段 + 环绕段的两段拷贝路径。
  #[test]
  fn clone_and_clone_from_wrapped_layout() {
    let mut q: VecDeque<u32> = VecDeque::new();
    q.reserve(4);
    for i in 0..4 {
      q.push_back(i);
    }
    // head=3, size=1；再 push 三个使逻辑序列物理上绕回 [3],[0],[1],[2]。
    q.pop_front();
    q.pop_front();
    q.pop_front();
    q.push_back(4);
    q.push_back(5);
    q.push_back(6);
    assert_eq!(q.iter().copied().collect::<Vec<_>>(), [3, 4, 5, 6]);
    assert_ne!(q.head, 0);

    let c = q.clone();
    assert_eq!(c.iter().copied().collect::<Vec<_>>(), [3, 4, 5, 6]);
    assert_eq!(c.head, q.head); // 拷贝构造保留 head
    assert_eq!(c.capacity(), q.capacity());

    let mut a: VecDeque<u32> = VecDeque::new();
    a.push_back(99);
    a.clone_from(&q);
    assert_eq!(a.iter().copied().collect::<Vec<_>>(), [3, 4, 5, 6]);
    assert_eq!(a.head, 0); // 赋值后总是 contiguous
    assert_eq!(a.capacity(), q.capacity());
  }
}
