use crate::{macros::luau_assert::LUAU_ASSERT, records::small_vector::SmallVector};

impl<T, const N: usize> SmallVector<T, N> {
  pub fn operator_index(&self, index: usize) -> &T {
    LUAU_ASSERT!(index < self.size() as usize);

    // cpp `operator[](index)`；内部存储由 `smallvec` 托管，经切片安全访问。
    &self.as_slice()[index]
  }
}
