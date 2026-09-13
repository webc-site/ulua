use ulua_ast::records::location::Location;

use crate::{
  functions::{
    follow_type::follow_type_id, get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_table_type::get_table_type, get_type_alt_j::get_type_id,
  },
  records::{
    any_type::AnyType, builtin_types::BuiltinTypes, generic_error::GenericError,
    type_error::TypeError,
  },
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId},
};

pub(crate) fn find_metatable_entry(
  builtin_types: *mut BuiltinTypes,
  errors: &mut ErrorVec,
  ty: TypeId,
  entry: &str,
  location: Location,
) -> Option<TypeId> {
  let ty = follow_type_id(ty);

  let metatable = get_metatable_type_id_not_null_builtin_types(ty, unsafe { &*builtin_types });
  metatable?;

  let unwrapped = follow_type_id(metatable.unwrap());

  let any_type = get_type_id::<AnyType>(unwrapped);
  if !any_type.is_none() {
    return Some(unsafe { (*builtin_types).any_type });
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
