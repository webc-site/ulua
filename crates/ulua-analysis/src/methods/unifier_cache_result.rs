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
        unsafe {
          (*self.shared_state).cached_unify.insert((sub_ty, super_ty));
        }
      }
    } else if self.errors.len() == prev_error_count + 1
      && self.unifier_can_cache_result(sub_ty, super_ty)
    {
      // C++: `sharedState.cachedUnifyError[{sub_ty, super_ty}] = errors.back().data;`
      let error_data = self.errors.last().unwrap().data.clone();
      unsafe {
        *(*self.shared_state)
          .cached_unify_error
          .get_or_insert((sub_ty, super_ty)) = error_data;
      }
    }
  }
}
