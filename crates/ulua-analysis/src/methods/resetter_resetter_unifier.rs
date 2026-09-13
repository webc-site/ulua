use crate::{enums::variance::Variance, records::resetter::Resetter};

impl Resetter {
  /// # Safety
  /// 调用方须保证 `variance` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn new(variance: *mut Variance) -> Self {
    let old_value = unsafe { *variance };
    Self {
      old_value,
      variance,
    }
  }
}
