use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{extra_information::ExtraInformation, type_error::TypeError},
  type_aliases::{
    error_vec::ErrorVec, module_name_type::ModuleName, type_error_data::TypeErrorData,
    type_id::TypeId,
  },
};
pub fn report_available_overloads(
  errors: &mut ErrorVec,
  location: Location,
  module_name: &ModuleName,
  overloads: &[TypeId],
) {
  if overloads.len() <= 1 {
    return;
  }

  let mut s = String::from("Available overloads: ");
  for (i, &ty) in overloads.iter().enumerate() {
    if i > 0 {
      s.push_str(if i == overloads.len() - 1 {
        "; and "
      } else {
        "; "
      });
    }
    s.push_str(&to_string_type_id(ty));
  }

  errors.push(TypeError::type_error_location_module_name_type_error_data(
    location,
    module_name.clone(),
    TypeErrorData::ExtraInformation(ExtraInformation::new(s)),
  ));
}
