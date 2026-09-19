use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get},
  records::{
    metatable_type::MetatableType, primitive_type::PrimitiveType, scope::Scope,
    subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult, table_type::TableType,
  },
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_mt: &MetatableType,
    super_prim: &PrimitiveType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if super_prim.r#type == PrimitiveType::TABLE {
      let followed = follow_type_id(sub_mt.table());
      if let Some(sub_table) = get::<TableType>(followed) {
        return self
          .is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(
            env, sub_table, super_prim, scope,
          );
      } else if let Some(sub_nested_mt) = get::<MetatableType>(followed) {
        return self
          .is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
            env,
            sub_nested_mt,
            super_prim,
            scope,
          );
      }
    }
    SubtypingResult {
      is_subtype: false,
      ..Default::default()
    }
  }
}
