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
    // C++ 中 normalize 失败返回空指针，进入 normalized 重载后命中等价分支
    // `{false, true}`（非子类型且归一化过于复杂）
    let (sub_norm, super_norm) = (
      unsafe { (*self.normalizer).try_normalize(sub_ty) },
      unsafe { (*self.normalizer).try_normalize(super_ty) },
    );
    let (Some(sub_norm), Some(super_norm)) = (sub_norm, super_norm) else {
      return SubtypingResult {
        is_subtype: false,
        normalization_too_complex: true,
        is_cacheable: true,
        is_error_suppressing: false,
        errors: Default::default(),
        reasoning: Default::default(),
        assumed_constraints: Default::default(),
        generic_bounds_mismatches: Default::default(),
      };
    };
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
