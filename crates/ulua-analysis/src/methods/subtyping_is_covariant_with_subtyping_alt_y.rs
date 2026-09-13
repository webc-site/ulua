use alloc::sync::Arc;

use crate::{
  enums::subtyping_suppression_policy::SubtypingSuppressionPolicy,
  records::{
    normalized_type::NormalizedType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  },
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_shared_ptr_normalized_type_shared_ptr_normalized_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_norm: &Arc<NormalizedType>,
    super_norm: &Arc<NormalizedType>,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
      env,
      sub_norm.tops,
      super_norm.tops,
      scope,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.booleans,
        super_norm.booleans,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );

    let mut extern_result = self.is_covariant_with_subtyping_environment_normalized_extern_type_normalized_extern_type_not_null_scope(env, &sub_norm.extern_types, &super_norm.extern_types, scope);
    extern_result.or_else(
      self.is_covariant_with_subtyping_environment_normalized_extern_type_type_ids_not_null_scope(
        env,
        &sub_norm.extern_types,
        &super_norm.tables,
        scope,
      ),
    );
    result.and_also(extern_result, SubtypingSuppressionPolicy::Any);

    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.errors,
        super_norm.errors,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.nils,
        super_norm.nils,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.numbers,
        super_norm.numbers,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );

    let mut string_result = self.is_covariant_with_subtyping_environment_normalized_string_type_normalized_string_type_not_null_scope(env, &sub_norm.strings, &super_norm.strings, scope);
    string_result.or_else(
      self.is_covariant_with_subtyping_environment_normalized_string_type_type_ids_not_null_scope(
        env,
        &sub_norm.strings,
        &super_norm.tables,
        scope,
      ),
    );
    result.and_also(string_result, SubtypingSuppressionPolicy::Any);

    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.threads,
        super_norm.threads,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.buffers,
        super_norm.buffers,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_ids_type_ids_not_null_scope(
        env,
        &sub_norm.tables,
        &super_norm.tables,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(self.is_covariant_with_subtyping_environment_normalized_function_type_normalized_function_type_not_null_scope(env, &sub_norm.functions, &super_norm.functions, scope), SubtypingSuppressionPolicy::Any);

    result
  }
}
