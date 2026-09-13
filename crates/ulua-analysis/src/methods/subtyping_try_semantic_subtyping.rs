use crate::{
  records::{
    scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult,
  },
  type_aliases::type_id::TypeId,
};

impl Subtyping {
  pub fn try_semantic_subtyping(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: *mut Scope,
    original: &mut SubtypingResult,
  ) -> SubtypingResult {
    let sub_norm = unsafe { (*self.normalizer).normalize(sub_ty) };
    let super_norm = unsafe { (*self.normalizer).normalize(super_ty) };
    let mut semantic = self
            .is_covariant_with_subtyping_environment_shared_ptr_normalized_type_shared_ptr_normalized_type_not_null_scope(
                env,
                &sub_norm,
                &super_norm,
                scope,
            );

    if semantic.normalization_too_complex {
      semantic
    } else if semantic.is_subtype {
      semantic.reasoning.clear();
      semantic
    } else {
      original.clone()
    }
  }
}
