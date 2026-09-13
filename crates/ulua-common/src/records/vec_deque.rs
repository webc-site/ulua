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
      unsafe {
        dealloc(p.as_ptr() as *mut u8, layout);
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
    new_deque.queue_size = self.queue_size;

    let (head_part, tail_part) = self.slices();
    // SAFETY：新缓冲区是刚分配的未初始化内存，容量与旧缓冲区相同，
    // 槽位一一对应，用 ptr::write 逐元素克隆写入。
    unsafe {
      for (i, val) in head_part.iter().enumerate() {
        ptr::write(new_buffer.as_ptr().add(self.head + i), val.clone());
      }
      for (i, val) in tail_part.iter().enumerate() {
        ptr::write(new_buffer.as_ptr().add(i), val.clone());
      }
    }
    new_deque
  }
}

/// 前向迭代器：按逻辑顺序（front → back）产出元素。
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

/// 可变前向迭代器：按逻辑顺序产出 `&mut T`。
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
