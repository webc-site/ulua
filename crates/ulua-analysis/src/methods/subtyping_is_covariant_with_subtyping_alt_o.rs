use ulua_common::FFlag;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    metatable_type::MetatableType, property_type::Property, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    table_type::TableType,
  },
};

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_metatable_type_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_mt: &MetatableType,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let sub_table_id = follow_type_id(sub_mt.table());
    if let Some(sub_table) = get_type_id::<TableType>(sub_table_id) {
      let sub_mt_id = follow_type_id(sub_mt.metatable());
      if let Some(sub_mt_table) = get_type_id::<TableType>(sub_mt_id)
        && let Some(index_prop) = sub_mt_table.props.get("__index")
        && let Some(read_ty) = index_prop.read_ty
      {
        let index_table_id = follow_type_id(read_ty);
        if let Some(index_table) = get_type_id::<TableType>(index_table_id) {
          let mut faux_sub_table = sub_table.clone();
          for (name, prop) in &index_table.props {
            if let Some(read_ty) = prop.read_ty
              && !faux_sub_table.props.contains_key(name)
            {
              faux_sub_table
                .props
                .insert(name.clone(), Property::readonly(read_ty));
            }
          }
          return if FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
            self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
              env,
              &faux_sub_table,
              super_table,
              false,
              scope,
            )
          } else {
            self.is_covariant_with_deprecated(env, &faux_sub_table, super_table, false, scope)
          };
        }
      }
      return if FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
        self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
          env,
          sub_table,
          super_table,
          false,
          scope,
        )
      } else {
        self.is_covariant_with_deprecated(env, sub_table, super_table, false, scope)
      };
    }
    SubtypingResult {
      is_subtype: false,
      ..Default::default()
    }
  }
}
