use crate::{
  enums::{subtyping_suppression_policy::SubtypingSuppressionPolicy, type_field::TypeField},
  records::{
    metatable_type::MetatableType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
  },
  type_aliases::component::Component,
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_metatable_type_metatable_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_mt: &MetatableType,
    super_mt: &MetatableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_mt.table(),
        super_mt.table(),
        scope,
      )
      .with_both_component(Component::TypeField(TypeField::Table))
      .and_also(
        self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            sub_mt.metatable(),
            super_mt.metatable(),
            scope,
          )
          .with_both_component(Component::TypeField(TypeField::Metatable))
          .to_owned(),
        SubtypingSuppressionPolicy::Any,
      )
      .to_owned()
  }
}
