//! @interface-stub
use ulua_analysis::{
  enums::normalization_result::NormalizationResult, records::normalized_type::NormalizedType,
};

use crate::records::normalize_fixture::NormalizeFixture;

impl NormalizeFixture {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn is_inhabited(&mut self, norm: *const NormalizedType) -> bool {
    if norm.is_null() {
      return false;
    }

    self.get_frontend();
    self
      .normalizer
      .as_mut()
      .expect("NormalizeFixture normalizer")
      .is_inhabited_normalized_type(unsafe { &*norm })
      == NormalizationResult::True
  }
}
