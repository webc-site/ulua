use core::slice::Iter;

use crate::records::temp_vector::TempVector;

impl<'a, T> TempVector<'a, T> {
  /// 按值区间 `[offset, offset + size_)` 迭代。
  ///
  /// 对应 C++ 的 `begin()/end()` 循环写法；TempVector 存活期间独占 scratch
  /// 尾部（Drop 断言 `storage.len() == offset + size_`），切片始终有效。
  #[inline]
  pub fn iter(&self) -> Iter<'_, T> {
    // SAFETY: storage 指向借用的 Vec，本引用仍存活；offset + size_ 由
    // push_back/Drop 维护在 Vec 长度之内。
    unsafe { (&*self.storage)[self.offset..self.offset + self.size_].iter() }
  }
}
