pub use find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_value_context_location_bool as find_table_property_respecting_meta;
use ulua_ast::records::location::Location;

use crate::{
  enums::value_context::ValueContext,
  functions::{
    find_metatable_entry::find_metatable_entry, first::first, follow_type::follow_type_id,
    follow_type_pack::follow_type_pack_id, get_table_type::get_table_type,
    get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    any_type::AnyType, builtin_types::BuiltinTypes, function_type::FunctionType,
    generic_error::GenericError, type_error::TypeError,
  },
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn find_table_property_respecting_meta_not_null_builtin_types_error_vec_type_id_string_value_context_location_bool(
  builtin_types: *mut BuiltinTypes,
  errors: &mut ErrorVec,
  ty: TypeId,
  name: &str,
  context: ValueContext,
  location: Location,
  use_new_solver: bool,
) -> Option<TypeId> {
  unsafe {
    let any_type = get_type_id::<AnyType>(ty);
    if !any_type.is_none() {
      return Some(ty);
    }

    let table_type = get_table_type(ty);
    if let Some(tt) = table_type
      && let Some(prop) = tt.props.get(name)
    {
      match context {
        ValueContext::RValue => return prop.read_ty,
        ValueContext::LValue => return prop.write_ty,
      }
    }

    let mut mt_index = find_metatable_entry(builtin_types, errors, ty, "__index", location);
    let mut count = 0;

    while let Some(index) = mt_index {
      if count >= 100 {
        return None;
      }
      count += 1;

      let index = follow_type_id(index);

      if let Some(itt) = get_table_type(index) {
        if let Some(fit) = itt.props.get(name) {
          if use_new_solver {
            match context {
              ValueContext::RValue => return fit.read_ty,
              ValueContext::LValue => return fit.write_ty,
            }
          } else {
            return fit.read_ty;
          }
        }
      } else if let Some(itf) = get_type_id::<FunctionType>(index) {
        let r = first(follow_type_pack_id(itf.ret_types), false);
        if let Some(r) = r {
          return Some(r);
        } else {
          return Some((*builtin_types).nil_type);
        }
      } else if get_type_id::<AnyType>(index).is_some() {
        return Some((*builtin_types).any_type);
      } else {
        let type_str = to_string_type_id(index);
        errors.push(TypeError::type_error_location_type_error_data(
          location,
          TypeErrorData::GenericError(GenericError::new(format!(
            "__index should either be a function or table. Got {}",
            type_str
          ))),
        ));
      }

      mt_index = find_metatable_entry(
        builtin_types,
        errors,
        mt_index.unwrap(),
        "__index",
        location,
      );
    }

    None
  }
}
