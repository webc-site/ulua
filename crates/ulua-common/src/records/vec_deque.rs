//! `Luau::VecDeque` 的安全移植 —— 以 `alloc::collections::VecDeque` 为承载。
//! Reference: `cpp/Common/include/Luau/VecDeque.h`（行为对照
//! `cpp/tests/VecDeque.test.cpp`）。
//!
//! 数据结构完全重写（上游的 `NonNull` 手搓环缓冲删除），但对外可观察行为
//! 保持不变：
//! - 增长策略保留 C++ 的 1.5x + 1（`grow` 先按目标容量 `reserve_exact`，
//!   std 内部的倍增不会触发；`reserve_exact` 对 ≥4 槽的请求给出精确容量，
//!   与 C++ `allocator::allocate(n)` 一致）。C++ 1.5x 增长因子是实现细节，
//!   此处仅为通过 cpp oracle 的容量断言而保留。
//! - `is_contiguous` 需要 front 元素的物理槽位 `head`；std 仅在容量变化
//!   （重排）时把 head 归零，与 C++ grow/reserve 的重排时刻一致，因此用
//!   一个 `head` 镜像即可等价表达，且本类型所有操作都在镜像上复刻 C++ 的
//!   head 算术（push_front 前移、pop_front 后移、重排归零）。
//! - `from_init_list` / `reserve` / `shrink_to_fit` 的容量语义同 C++；
//!   唯一偏差是 std 对小容量（< 4 槽）按最小桶取整，cpp 测试面（≥10 元素）
//!   不可观察。

use alloc::collections::{
  VecDeque as StdVecDeque,
  vec_deque::{IntoIter as StdIntoIter, Iter as StdIter, IterMut as StdIterMut},
};
use core::{
  fmt::{Debug, Formatter, Result as FmtResult},
  mem::size_of,
  ptr::eq,
};

use crate::macros::luau_assert::LUAU_ASSERT;

/// C++ 增长因子 1.5x（folly FBVector 风格，便于内存复用、减少碎片）。
const GROWTH_FACTOR_NUM: usize = 3;
const GROWTH_FACTOR_DEN: usize = 2;
/// 空容器首次增长的容量（C++ grow 的常数项）。
const INITIAL_CAPACITY: usize = 4;

/// C++ `Luau::VecDeque<T>`：双端队列。方法名与语义逐一对齐
/// `VecDeque.h`（`size`/`empty`/`push_back`/`pop_front`/`at`/…）。
pub struct VecDeque<T> {
  /// 承载：std 环缓冲负责全部内存管理（无裸指针、无手动 alloc/dealloc）。
  inner: StdVecDeque<T>,
  /// front 元素的物理槽位镜像（仅服务 `is_contiguous` 的 C++ 语义；
  /// 见模块文档）。std 内部重排（容量变化）时归零，本类型在每次容量变化
  /// 后同步复位，恒与内部布局一致。
  head: usize,
}

impl<T> VecDeque<T> {
  /// C++ `VecDeque()` 默认构造（空容器，未分配缓冲区）。
  pub fn new() -> Self {
    Self {
      inner: StdVecDeque::new(),
      head: 0,
    }
  }

  /// 已分配的槽位总数（C++ `capacity()`）。
  pub fn capacity(&self) -> usize {
    self.inner.capacity()
  }

  /// 当前元素个数（C++ `size()`）。
  pub fn size(&self) -> usize {
    self.inner.len()
  }

  /// 容器是否为空（C++ `empty()`）。
  pub fn empty(&self) -> bool {
    self.inner.is_empty()
  }

  /// 容器是否已满（C++ `isFull()`）。全仓仅本类型 push 路径消费，降 `pub(crate)`。
  pub(crate) fn is_full(&self) -> bool {
    self.inner.len() == self.capacity()
  }

  /// 逻辑序列是否在缓冲区中连续、未环绕（C++ `isContiguous()`，
  /// 溢出安全形式 `head <= capacity - size`）。
  pub fn is_contiguous(&self) -> bool {
    self.head <= self.capacity() - self.inner.len()
  }

  /// 可分配容量的硬上限（C++ `maxSize()` 的 `MAX_SIZE / sizeof(T)`）。
  pub fn max_size(&self) -> usize {
    // ZST 无存储开销，容量仅受 usize 索引域限制（C++ 的除法对 ZST 是除零
    // UB，此处收敛为饱和值而非 panic）。
    usize::MAX.checked_div(size_of::<T>()).unwrap_or(usize::MAX)
  }

  /// 越界即 `LUAU_ASSERT`（debug）/ panic（release，收敛 C++ 的未定义行为）
  /// 的下标读，忠实镜像 C++ `operator[]`（C++ `VecDeque.h`）；需要 panic
  /// 边界的版本用 [`VecDeque::at`]。
  #[inline]
  pub fn operator_index(&self, pos: usize) -> &T {
    LUAU_ASSERT!(pos < self.size());
    &self.inner[pos]
  }

  /// `VecDeque(std::initializer_list<T>)` — 按元素数精确预留（C++ 的
  /// `allocate(init.size())`），缓冲区从 0 号槽起连续填充
  /// （`capacity == size`，无增长）。
  pub fn from_init_list(items: Vec<T>) -> Self {
    let mut inner = StdVecDeque::new();
    if !items.is_empty() {
      // ≥4 槽时 reserve_exact 给出精确容量（std 最小桶取整只影响 <4 的
      // 小队列，cpp 测试面不可观察）。
      inner.reserve_exact(items.len());
      inner.extend(items);
    }
    Self { inner, head: 0 }
  }

  /// C++ 增长策略：`1.5x + 1`，空容器取 [`INITIAL_CAPACITY`]。
  /// release（无溢出检查）下 `old_capacity * 3` 可回绕成小值，绕过
  /// [`VecDeque::grow`] 的上限检查后以错误容量重分配；`checked_mul` 收口，
  /// 返回 `None` 表示请求容量必然超上限，由 `grow` 按 `max_size` 分支处理。
  fn grow_capacity(&self) -> Option<usize> {
    let old_capacity = self.capacity();
    if old_capacity == 0 {
      return Some(INITIAL_CAPACITY);
    }
    let tripled = old_capacity.checked_mul(GROWTH_FACTOR_NUM)?;
    Some(tripled / GROWTH_FACTOR_DEN + 1)
  }

  /// cpp `grow`：按 1.5x + 1 重排到新缓冲区，front 归零（C++ 的
  /// `head = 0`）。std 侧由 `reserve_exact` 一次性给足目标容量，其内部
  /// 倍增分支不会触发；std 重排同样把元素搬回 0 号槽起，镜像同步归零。
  fn grow(&mut self) {
    // `grow_capacity` 溢出（None）意味着容量请求已不可表示，与超过
    // `max_size` 同语义：一律走 `bad_array_new_length` 分支。
    match self.grow_capacity() {
      Some(cap) if cap <= self.max_size() => {
        // 调用点仅在满（len == capacity）时进入，目标容量必超当前，
        // `cap - len` 无下溢。
        self.inner.reserve_exact(cap - self.inner.len());
        self.head = 0;
      }
      _ => panic!("bad_array_new_length"),
    }
  }

  /// `push_back`（C++ `push_back(const T&)`）。满时先按 C++ 策略增长。
  pub fn push_back(&mut self, value: T) {
    if self.is_full() {
      self.grow();
    }
    self.inner.push_back(value);
  }

  /// `pop_back`（C++ `pop_back`）：析构末元素，无返回值。
  pub fn pop_back(&mut self) {
    assert!(!self.empty());
    self.inner.pop_back();
  }

  /// `push_front`（C++ `push_front(const T&)`）：head 前移（回绕）后写入。
  pub fn push_front(&mut self, value: T) {
    if self.is_full() {
      self.grow();
    }
    self.head = if self.head == 0 {
      self.capacity() - 1
    } else {
      self.head - 1
    };
    self.inner.push_front(value);
  }

  /// `pop_front`（C++ `pop_front`）：析构首元素，head 后移（回绕）。
  pub fn pop_front(&mut self) {
    assert!(!self.empty());
    self.inner.pop_front();
    self.head += 1;
    if self.head == self.capacity() {
      self.head = 0;
    }
  }

  /// `clear`（C++ `clear`）：析构全部元素，head 归零，保留容量。
  pub fn clear(&mut self) {
    self.inner.clear();
    self.head = 0;
  }

  /// `reserve(new_capacity)`（C++ 绝对容量语义）：请求更大容量时重排到
  /// 恰好 `new_capacity` 槽的新缓冲区，front 归零；否则 no-op。
  pub fn reserve(&mut self, new_capacity: usize) {
    if new_capacity > self.max_size() {
      panic!("too large");
    }
    if new_capacity <= self.capacity() {
      return;
    }
    // 目标容量 > 当前容量 ≥ len，减法无下溢；std 给出精确容量（≥4 时）。
    self.inner.reserve_exact(new_capacity - self.inner.len());
    self.head = 0;
  }

  /// `shrink_to_fit`（C++）：容量大于 size 时收缩到恰好 size 槽
  /// （size 为 0 时释放缓冲区），front 归零；容量已等于 size 时 no-op。
  /// std 的 `shrink_to_fit` 会舍入到 2 的幂桶，不满足 C++ 的精确容量语义，
  /// 故用 `reserve_exact` 迁槽（≥4 元素时给出精确容量，与 cpp 一致）。
  pub fn shrink_to_fit(&mut self) {
    let len = self.inner.len();
    if self.capacity() == len {
      return;
    }
    let mut shrunk: StdVecDeque<T> = StdVecDeque::new();
    shrunk.reserve_exact(len);
    shrunk.extend(self.inner.drain(..));
    self.inner = shrunk;
    self.head = 0;
  }

  /// `at(pos)`（C++ 带界检查的下标读）：越界 panic `"VecDeque out of range"`。
  pub fn at(&self, pos: usize) -> &T {
    if pos >= self.size() {
      panic!("VecDeque out of range");
    }
    &self.inner[pos]
  }

  /// `front`（C++ `front()`）：空容器 panic（C++ 为断言 + UB）。
  pub fn front(&self) -> &T {
    self.inner.front().expect("front on empty VecDeque")
  }

  /// [`VecDeque::front`] 的可变版本（C++ 非 const `front()`）。
  pub fn front_mut(&mut self) -> &mut T {
    self.inner.front_mut().expect("front on empty VecDeque")
  }

  /// `back`（C++ `back()`）：空容器 panic（C++ 为断言 + UB）。
  pub fn back(&self) -> &T {
    self.inner.back().expect("back on empty VecDeque")
  }

  /// 按逻辑顺序（front → back）迭代元素（C++ 语义与
  /// `std::collections::VecDeque::iter` 一致）。
  pub fn iter(&self) -> Iter<'_, T> {
    self.inner.iter()
  }

  /// [`VecDeque::iter`] 的可变版本。
  ///
  /// b28 零消费点裁定 (b) 保留 `pub`：返回类型 `IterMut` 同时嵌在
  /// `impl IntoIterator for &mut VecDeque` 公共接口中，与 `&`/by-value 两枚
  /// 组成 cpp 容器 range-for 平价的三件套镜像，降级须整删惯用法面，不为降而降。
  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    self.inner.iter_mut()
  }
}

impl<T> Default for VecDeque<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Debug> Debug for VecDeque<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "VecDeque")?;
    f.debug_list().entries(self.iter()).finish()
  }
}

impl<T: Clone> Clone for VecDeque<T> {
  /// C++ 拷贝构造：保留容量与 head 布局（is_contiguous 状态随 head 保留）。
  /// 用 `reserve_exact` 而非 `with_capacity`，后者会舍入到 2 的幂桶。
  fn clone(&self) -> Self {
    let mut inner = StdVecDeque::new();
    inner.reserve_exact(self.capacity());
    inner.extend(self.inner.iter().cloned());
    Self {
      inner,
      head: self.head,
    }
  }

  /// C++ 拷贝赋值 `operator=（const VecDeque&）`：容量不足以容纳
  /// `other.size()` 时按源容量重分配，`head` 归零——赋值后的队列总是
  /// contiguous，与拷贝构造保留 `head` 的行为不同。Reference: `VecDeque.h:202-236`。
  fn clone_from(&mut self, other: &Self) {
    // C++ `operator=` 的自赋值守卫：清空后无法再从自身取源。
    if eq(self, other) {
      return;
    }
    if self.capacity() < other.size() {
      let mut fresh: StdVecDeque<T> = StdVecDeque::new();
      fresh.reserve_exact(other.capacity());
      self.inner = fresh;
    }
    self.inner.clear();
    self.inner.extend(other.inner.iter().cloned());
    self.head = 0;
  }
}

/// 前向迭代器：按逻辑顺序（front → back）产出元素。直接别名 std 环缓冲
/// 迭代器——newtype 的逐方法转发（`Iterator`/`DoubleEndedIterator`/
/// `ExactSizeIterator`/`FusedIterator`）是纯样板，std 类型全量提供同一语义，
/// 保留原公开类型名即可。
pub type Iter<'a, T> = StdIter<'a, T>;

/// [`Iter`] 的可变版本。b28 裁定 (b) 保留 `pub`（公共 `IntoIterator` 接口嵌用，
/// 见 [`VecDeque::iter_mut`] 文档）。
pub type IterMut<'a, T> = StdIterMut<'a, T>;

/// 所有权迭代器：按逻辑顺序（front → back）消费元素；未消费的元素随
/// 迭代器一起析构。`len` 经 `ExactSizeIterator` 提供。
pub type IntoIter<T> = StdIntoIter<T>;

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
    self.inner.into_iter()
  }
}

// §8：断言读 `VecDeque` 的私有物理槽位游标 `head`（公开 API 只见等价元素序列），
// 迁 tests/ 需泄 pub，保留 src。
#[cfg(test)]
mod tests {
  use alloc::vec::Vec;

  use super::VecDeque;

  /// 环绕布局（head 在缓冲区中段、逻辑序列跨越物理末尾）下的克隆：拷贝
  /// 构造保留 head，`clone_from` 归零（contiguous）。回归覆盖 Clone 写入
  /// head 段 + 环绕段的两段拷贝路径。
  ///
  /// 留在 `src` 的理由：断言对象是私有字段——物理槽位游标 `head`，
  /// 外部测试无从读取（公开 API 只能看到等价的元素序列）。
  #[test]
  fn clone_and_clone_from_wrapped_layout() {
    let mut q: VecDeque<u32> = VecDeque::new();
    q.reserve(4);
    // 本 VecDeque 未实现 Extend，逐元素 push_back。
    (0..4).for_each(|v| q.push_back(v));
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
