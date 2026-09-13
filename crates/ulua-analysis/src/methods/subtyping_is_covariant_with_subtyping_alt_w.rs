use ulua_common::FFlag;

use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, type_field::TypeField},
  records::{
    scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, table_indexer::TableIndexer,
  },
  type_aliases::component::Component,
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_indexer: &TableIndexer,
    super_indexer: &TableIndexer,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if FFlag::LuauReadOnlyIndexers.get() {
      let mut result = SubtypingResult {
        is_subtype: false,
        ..Default::default()
      };

      if sub_indexer.is_read_only && !super_indexer.is_read_only {
        result.with_both_component(Component::TypeField(TypeField::IndexResult));
        return result;
      }

      result = self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
        env,
        sub_indexer.index_type,
        super_indexer.index_type,
        scope,
      );
      result.with_both_component(Component::TypeField(TypeField::IndexLookup));

      let mut value_result = if super_indexer.is_read_only {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env,
          sub_indexer.index_result_type,
          super_indexer.index_result_type,
          scope,
        )
      } else {
        self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
          env,
          sub_indexer.index_result_type,
          super_indexer.index_result_type,
          scope,
        )
      };
      value_result.with_both_component(Component::TypeField(TypeField::IndexResult));
      result.and_also(value_result, SubtypingSuppressionPolicy::Any);

      result
    } else {
      let mut result = self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
        env,
        sub_indexer.index_type,
        super_indexer.index_type,
        scope,
      );
      result.with_both_component(Component::TypeField(TypeField::IndexLookup));

      let mut value_result = self
        .is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
          env,
          sub_indexer.index_result_type,
          super_indexer.index_result_type,
          scope,
        );
      value_result.with_both_component(Component::TypeField(TypeField::IndexResult));
      result.and_also(value_result, SubtypingSuppressionPolicy::Any);
      result
    }
  }
}
