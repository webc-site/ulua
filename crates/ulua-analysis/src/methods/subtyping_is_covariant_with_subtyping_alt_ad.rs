use crate::records::{
  normalized_function_type::NormalizedFunctionType, scope::Scope, subtyping::Subtyping,
  subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_normalized_function_type_normalized_function_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_function: &NormalizedFunctionType,
    super_function: &NormalizedFunctionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if sub_function.is_never() || super_function.is_top {
      SubtypingResult {
        is_subtype: true,
        ..Default::default()
      }
    } else {
      self.is_covariant_with_subtyping_environment_type_ids_type_ids_not_null_scope(
        env,
        &sub_function.parts,
        &super_function.parts,
        scope,
      )
    }
  }
}
