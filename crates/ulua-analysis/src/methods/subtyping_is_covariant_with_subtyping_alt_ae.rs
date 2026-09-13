use crate::{
  enums::subtyping_suppression_policy::SubtypingSuppressionPolicy,
  records::{
    scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, type_ids::TypeIds,
  },
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_type_ids_type_ids_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_types: &TypeIds,
    super_types: &TypeIds,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };

    for sub_ty in &sub_types.order {
      let mut inner_result = SubtypingResult::default();

      for super_ty in &super_types.order {
        let candidate = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env, *sub_ty, *super_ty, scope,
          );
        inner_result.or_else(candidate);

        if inner_result.normalization_too_complex {
          return SubtypingResult {
            is_subtype: false,
            normalization_too_complex: true,
            ..Default::default()
          };
        }
      }

      result.and_also(inner_result, SubtypingSuppressionPolicy::Any);
    }

    result
  }
}
