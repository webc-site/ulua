//! `DenseHashTable::const_iterator` — 按占用位图升序产出 `&Item` 的前向迭代器。
//! Reference: `cpp/Common/include/Luau/DenseHash.h:507-560`：cpp 侧成员是
//! `(表指针, BitSet::iterator)`，`operator*` 用 `*bucketIt` 索引 `data`；Rust 侧
//! 把"表指针"换成切片借用，语义一致。
//!
//! map/set 包装层把产出的 `&Item` 改写成 `&Key` / `(&Key, &Value)`。

use core::iter::FusedIterator;

use crate::records::dense_hash_table::Bits;

/// b28 裁定 (b) 保留 `pub`：`DenseHashSet::iter`（pub）的返回类型，下游经类型
/// 推断消费（如 analysis 对 `require_set` 的 range 迭代），token 扫描不可见。
pub struct ConstIterator<'a, I> {
  pub(crate) items: &'a [I],
  pub(crate) buckets: Bits<'a>,
}

impl<'a, I> Iterator for ConstIterator<'a, I> {
  type Item = &'a I;

  #[inline]
  fn next(&mut self) -> Option<&'a I> {
    self.buckets.next().map(|bucket| &self.items[bucket])
  }
}

/// 位图迭代器耗尽后不会再产出，`FusedIterator` 让 `collect` 等下游省掉重复探测。
impl<I> FusedIterator for ConstIterator<'_, I> {}
