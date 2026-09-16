use crate::records::small_vector::SmallVector;

impl<T, const N: usize> SmallVector<T, N> {
  pub fn end(&self) -> *const T {
    // 一 past-end 指针：`as_ptr_range().end` 安全构造，与 C++ `end()` 一致。
    self.as_slice().as_ptr_range().end
  }
}
