use core::ptr::null;

use crate::{
  enums::subtyping_suppression_policy::SubtypingSuppressionPolicy,
  functions::lookup_extern_type_prop::lookup_extern_type_prop,
  records::{
    extern_type::ExternType, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};
impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_type_id_extern_type_type_id_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    sub_extern_type: &ExternType,
    super_ty: TypeId,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: true,
      ..Default::default()
    };

    *env.substitutions.get_or_insert(super_ty) = sub_ty;

    for (name, prop) in &super_table.props {
      if let Some(prop_ref) = lookup_extern_type_prop(sub_extern_type, name) {
        result.and_also(
          self
            .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
              env,
              prop_ref,
              prop,
              name.as_ref(),
              false,
              scope,
            ),
          SubtypingSuppressionPolicy::Any,
        );
      } else {
        result.is_subtype = false;
        break;
      }
    }

    if let (Some(sub_indexer), Some(super_indexer)) = (
      sub_extern_type.indexer.as_ref(),
      super_table.indexer.as_ref(),
    ) {
      result.and_also(
        self.is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
          env,
          sub_indexer,
          super_indexer,
          scope,
        ),
        SubtypingSuppressionPolicy::Any,
      );
    } else if super_table.indexer.is_some() && sub_extern_type.indexer.is_none() {
      result.is_subtype = false;
    }

    *env.substitutions.get_or_insert(super_ty) = null();

    result
  }
}
