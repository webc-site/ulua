use ulua_ast::records::location::Location;

use crate::{
  functions::{
    follow_type, get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_table_type::get_table_type, get_type,
  },
  records::{
    any_type::AnyType, arena_handle::Handle, builtin_types::BuiltinTypes,
    generic_error::GenericError, type_error::TypeError,
  },
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId},
};

pub(crate) fn find_metatable_entry(
  builtin_types: Handle<BuiltinTypes>,
  errors: &mut ErrorVec,
  ty: TypeId,
  entry: &str,
  location: Location,
) -> Option<TypeId> {
  let ty = follow_type::follow(ty);

  // `metatable?`（None 即返 None）单句绑定，后续无 unwrap。
  let metatable = get_metatable_type_id_not_null_builtin_types(ty, builtin_types.get())?;

  let unwrapped = follow_type::follow(metatable);

  let any_type = get_type::get::<AnyType>(unwrapped);
  if any_type.is_some() {
    return Some(builtin_types.get().any_type);
  }

  let mtt = get_table_type(unwrapped);
  let mtt = match mtt {
    Some(t) => t,
    None => {
      errors.push(TypeError::type_error_location_type_error_data(
        location,
        TypeErrorData::GenericError(GenericError::new("Metatable was not a table".to_string())),
      ));
      return None;
    }
  };

  if let Some(prop_val) = mtt.props.get(entry) {
    if let Some(read_ty) = prop_val.read_ty {
      return Some(read_ty);
    } else if let Some(write_ty) = prop_val.write_ty {
      return Some(write_ty);
    }
  }

  None
}
