use crate::{records::unifier::Unifier, type_aliases::type_id::TypeId};

impl Unifier {
  pub fn unifier_cache_result(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    prev_error_count: usize,
  ) {
    if self.errors.len() == prev_error_count {
      if self.unifier_can_cache_result(sub_ty, super_ty) {
        // shared_state 为 `Handle` 单例句柄（契约见 records/arena_handle.rs），
        // insert 的临时可变借用止于语句末。
        self
          .shared_state
          .get_mut()
          .cached_unify
          .insert((sub_ty, super_ty));
      }
    } else if self.errors.len() == prev_error_count + 1
      && self.unifier_can_cache_result(sub_ty, super_ty)
    {
      // C++: `sharedState.cachedUnifyError[{sub_ty, super_ty}] = errors.back().data;`
      // 链上 `errors.len() == prev_error_count + 1` 蕴含 errors 非空。
      let error_data = self
        .errors
        .last()
        .expect("链上 len()==prev+1>=1 判定蕴含非空")
        .data
        .clone();
      // 同一 shared_state 句柄契约；get_or_insert 与覆写的临时可变借用
      // 只覆盖本条赋值语句，error_data 是刚 clone 的本地拥有值。
      *self
        .shared_state
        .get_mut()
        .cached_unify_error
        .get_or_insert((sub_ty, super_ty)) = error_data;
    }
  }
}
