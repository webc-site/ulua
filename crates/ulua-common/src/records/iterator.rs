//! `DenseHashTable::iterator` — `const_iterator` 的可变版本，按占用位图升序产出
//! `&mut Item`。Reference: `cpp/Common/include/Luau/DenseHash.h:562-615`。
//!
//! cpp 里它和 const 版只差 item 类型；Rust 侧不能一边以 `&mut` 取 `data`、一边
//! 以 `&` 读整张表，于是 `DenseHashTable::iter_mut` 把两个字段分别借用后装箱进
//! 本类型：`items` 可变、`buckets`（位图）只读。位图按升序产出桶号，因此
//! `items` 只需单调前进（`IterMut::nth` 跳到目标槽），不重走空槽、也不比较键。

use core::{iter::FusedIterator, slice::IterMut};

use crate::records::dense_hash_table::Bits;

/// `DenseHashTable::iter_mut`（`pub(crate)`）的游标，随唯一门面函数降
/// `pub(crate)`（b28 零消费点收口）。
pub(crate) struct MutIterator<'a, I> {
  pub(crate) items: IterMut<'a, I>,
  pub(crate) buckets: Bits<'a>,
  /// `items` 已消费到的槽号（= 下一次 `nth` 的基准），用于把升序桶号换算成增量。
  pub(crate) consumed: usize,
}

impl<'a, I> Iterator for MutIterator<'a, I> {
  type Item = &'a mut I;

  #[inline]
  fn next(&mut self) -> Option<&'a mut I> {
    let bucket = self.buckets.next()?;
    debug_assert!(bucket >= self.consumed, "占用位图必须升序产出桶号");
    let item = self.items.nth(bucket - self.consumed);
    self.consumed = bucket + 1;
    item
  }
}

impl<I> FusedIterator for MutIterator<'_, I> {}
