use ulua_analysis::{
  enums::normalization_result::NormalizationResult, records::normalized_type::NormalizedType,
};

use crate::records::normalize_fixture::NormalizeFixture;

impl NormalizeFixture {
  /// cpp `isInhabited(NormalizedType*)`：空句柄直接判否（对齐 cpp 的 `if (!ty) return false`）。
  ///
  /// (a) 类裸指针入参已收口为 `Option<&NormalizedType>`：cpp 判空分支由
  /// `Option::None` 表达；入参存活期交给引用本身（调用方由
  /// `to_normalized_type` 返回的 `Option<Arc<NormalizedType>>` 借用而来），
  /// 下游 `is_inhabited_normalized_type` 为安全只读查询，本函数零 `unsafe`。
  pub fn is_inhabited(&mut self, norm: Option<&NormalizedType>) -> bool {
    let Some(norm) = norm else {
      return false;
    };

    self.get_frontend();
    self
      .normalizer
      .as_mut()
      .expect("NormalizeFixture normalizer")
      .is_inhabited_normalized_type(norm)
      == NormalizationResult::True
  }
}
