use crate::{
  enums::subtyping_suppression_policy::SubtypingSuppressionPolicy,
  records::{
    normalized_string_type::NormalizedStringType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    type_ids::TypeIds,
  },
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_normalized_string_type_type_ids_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_string: &NormalizedStringType,
    super_tables: &TypeIds,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if sub_string.is_never() {
      return SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
    }

    if sub_string.is_cofinite {
      let mut result = SubtypingResult::default();
      for super_table in &super_tables.order {
        let candidate = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            unsafe { (*self.builtin_types).string_type },
            *super_table,
            scope,
          );
        result.or_else(candidate);
        if result.is_subtype {
          return result;
        }
      }
      return result;
    }

    for super_table in &super_tables.order {
      let mut result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
      for sub_string in sub_string.singletons.values() {
        let candidate = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            *sub_string,
            *super_table,
            scope,
          );
        result.and_also(candidate, SubtypingSuppressionPolicy::Any);
        if !result.is_subtype {
          break;
        }
      }

      if result.is_subtype {
        return result;
      }
    }

    SubtypingResult {
      is_subtype: false,
      ..Default::default()
    }
  }
}
