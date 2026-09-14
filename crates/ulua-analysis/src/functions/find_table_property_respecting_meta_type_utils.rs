use ulua_ast::records::location::Location;

use crate::{
  enums::value_context::ValueContext,
  functions::find_table_property_respecting_meta_type_utils_alt_b::find_table_property_respecting_meta,
  records::builtin_types::BuiltinTypes,
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};

pub(crate) fn find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_location_bool(
  builtin_types: *mut BuiltinTypes,
  errors: &mut ErrorVec,
  ty: TypeId,
  name: &str,
  location: Location,
  use_new_solver: bool,
) -> Option<TypeId> {
  unsafe {
    find_table_property_respecting_meta(
      builtin_types,
      errors,
      ty,
      name,
      ValueContext::RValue,
      location,
      use_new_solver,
    )
  }
}
