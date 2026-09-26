use crate::{
  records::{subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult},
  type_aliases::type_id::TypeId,
};

impl SubtypingEnvironment {
  pub fn try_find_subtyping_result(
    &self,
    sub_and_super: (TypeId, TypeId),
  ) -> Option<&SubtypingResult> {
    if let Some(it) = self.seen_set_cache.find(&sub_and_super) {
      return Some(it);
    }

    if !self.parent.is_null() {
      return unsafe { (*self.parent).try_find_subtyping_result(sub_and_super) };
    }

    None
  }
}
