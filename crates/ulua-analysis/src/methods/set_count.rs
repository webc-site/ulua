//! Source: `Analysis/include/Luau/Set.h:93-97` (hand-ported)

use alloc::string::String;
use core::hash::Hash;

use crate::records::set::Set;
impl<T: Clone + Hash + PartialEq> Set<T> {
  /// C++ `size_t count(const T& element) const`.
  pub fn count(&self, element: &T) -> usize {
    match self.mapping.find(element) {
      Some(entry) if *entry => 1,
      _ => 0,
    }
  }
}

/// `Set<String>` 的 `&str` 借用视图查询口（r7-tset1；实形裁定与整体论证见
/// [`set_insert_set`](super::set_insert_set) 专化块文档）。零克隆、零临时分配：
/// 直用 `DenseHashMap::find_str` 借用核，与泛型口 `count` 判定路径逐位一致
/// （`String`/`str` 哈希与等值在默认 functor 下逐位相同）。旧测试形
/// `count(&String::from(x))` 每次调用现场物化 1 malloc，本口降为 0。
///
/// [`set_insert_set`]: super::set_insert_set
impl Set<String> {
  /// `count` 的 `&str` 借用口。
  pub fn count_str(&self, s: &str) -> usize {
    match self.mapping.find_str(s) {
      Some(entry) if *entry => 1,
      _ => 0,
    }
  }
}
