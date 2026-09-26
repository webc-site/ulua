use alloc::{string::String, sync::Arc};

use crate::{
  records::{type_error::TypeError, type_mismatch::TypeMismatch, unifier::Unifier},
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  /// `tryUnify*` 各族「wanted/given 成对报 TypeMismatch（现取 mismatch context）」
  /// 骨架单点，带 reason/error 参数的 ext 版。收口前该块在
  /// `unifier_try_unify_normalized_types.rs` 以 `report_normalized_mismatch` 手抄，
  /// 本函数展开与收口前逐字段等价（`unifier_mismatch_context` 先于 `report_error`
  /// 求值的顺序亦保持；`error.map(Arc::new)` 同款）。
  pub(crate) fn unifier_report_type_mismatch_ext(
    &mut self,
    wanted_type: TypeId,
    given_type: TypeId,
    reason: String,
    error: Option<TypeError>,
  ) {
    let context = self.unifier_mismatch_context();
    self.report_error_location_type_error_data(
      self.location,
      TypeErrorData::TypeMismatch(TypeMismatch {
        wanted_type,
        given_type,
        reason,
        error: error.map(Arc::new),
        context,
      }),
    );
  }

  /// `tryUnify*` 各族「wanted/given 直接成对报 TypeMismatch（空 reason、无 error、
  /// 现取 mismatch context）」骨架单点，委托 ext 版填默认值。
  pub(crate) fn unifier_report_type_mismatch(&mut self, wanted_type: TypeId, given_type: TypeId) {
    self.unifier_report_type_mismatch_ext(wanted_type, given_type, String::new(), None)
  }
}
