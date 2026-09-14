use alloc::vec::Vec;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_type_alt_j::get_type_id,
  },
  methods::{path_builder_build::PathBuilderBuild, path_builder_mt::PathBuilderMt},
  records::{
    path_builder::PathBuilder, primitive_type::PrimitiveType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    table_type::TableType,
  },
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_primitive_type_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_prim: &PrimitiveType,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: false,
      ..Default::default()
    };

    if sub_prim.r#type == PrimitiveType::STRING {
      // SAFETY: builtin_types 裸指针由构造契约保证有效；解引用不借用 self。
      let bt = unsafe { &*self.builtin_types };
      if let Some(metatable) = get_metatable_type_id_not_null_builtin_types(bt.string_type, bt)
        && let Some(mttv) = get_type_id::<TableType>(follow_type_id(metatable))
        && let Some(it) = mttv.props.get("__index")
      {
        // the `string` metatable should not have any write-only types.
        LUAU_ASSERT!(!it.read_ty.unwrap().is_null());

        if let Some(string_table) = get_type_id::<TableType>(it.read_ty.unwrap()) {
          if FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
            let mut sub_result = self
              .is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
                env,
                string_table,
                super_table,
                false,
                scope,
              );
            sub_result.with_sub_path(
              PathBuilder {
                components: Vec::new(),
              }
              .mt()
              .read_prop("__index")
              .build(),
            );
            result.or_else(sub_result);
          } else {
            let mut sub_result =
              self.is_covariant_with_deprecated(env, string_table, super_table, false, scope);
            sub_result.with_sub_path(
              PathBuilder {
                components: Vec::new(),
              }
              .mt()
              .read_prop("__index")
              .build(),
            );
            result.or_else(sub_result);
          }
        }
      }
    } else if sub_prim.r#type == PrimitiveType::TABLE {
      let is_subtype = super_table.props.is_empty()
        && (super_table.indexer.is_none() || super_table.state == TableState::Generic);
      result.is_subtype = is_subtype;
      return result;
    }

    result
  }
}
