use alloc::vec::Vec;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    follow_type::follow_type_id, get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id,
  },
  methods::{path_builder_build::PathBuilderBuild, path_builder_mt::PathBuilderMt},
  records::{
    path_builder::PathBuilder, scope::Scope, singleton_type::SingletonType,
    string_singleton::StringSingleton, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    table_type::TableType,
  },
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_singleton_type_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_singleton: &SingletonType,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: false,
      ..Default::default()
    };

    // SAFETY: builtin_types 裸指针由构造契约保证有效；解引用不借用 self。
    let bt = unsafe { &*self.builtin_types };
    if get_singleton_type::<StringSingleton>(sub_singleton).is_some()
      && let Some(metatable) = get_metatable_type_id_not_null_builtin_types(bt.string_type, bt)
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
          let mut pb = PathBuilder {
            components: Vec::new(),
          };
          pb.mt();
          pb.read_prop("__index");
          sub_result.with_sub_path(pb.build());
          result.or_else(sub_result);
        } else {
          let mut sub_result =
            self.is_covariant_with_deprecated(env, string_table, super_table, false, scope);
          let mut pb = PathBuilder {
            components: Vec::new(),
          };
          pb.mt();
          pb.read_prop("__index");
          sub_result.with_sub_path(pb.build());
          result.or_else(sub_result);
        }
      }
    }

    result
  }
}
