//! `VecDeque`（cpp `Common/include/Luau/VecDeque.h`，原型
//! `cpp/tests/VecDeque.test.cpp`）中移植特有的 ZST 路径。
//!
//! cpp 侧元素类型恒为 `int`，ZST 分支不存在；Rust 里 `size_of::<T>() == 0`
//! 会让 `maxSize()` 的除法变成除零、缓冲区必须退化成 dangling 指针
//! （`alloc`/`dealloc` 禁止零大小 layout）。这两处是移植新增的分支，
//! 只能用一个真正的 ZST 元素类型来钉。

use ulua_common::records::vec_deque::VecDeque;

#[test]
fn zst_push_pop_grow_iter() {
  let mut q: VecDeque<()> = VecDeque::new();
  assert_eq!(q.max_size(), usize::MAX); // 除零分支收敛为饱和值
  for _ in 0..50 {
    q.push_back(());
    q.push_front(());
  }
  assert_eq!(q.size(), 100);
  assert_eq!(q.iter().count(), 100);
  assert_eq!(q.into_iter().count(), 100);

  let mut r: VecDeque<()> = VecDeque::new();
  r.push_back(());
  r.pop_back();
  assert!(r.empty());
}
