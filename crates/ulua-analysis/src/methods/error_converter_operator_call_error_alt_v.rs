use alloc::string::String;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{
    cannot_call_non_function::CannotCallNonFunction, error_converter::ErrorConverter,
    function_type::FunctionType, primitive_type::PrimitiveType, union_type::UnionType,
  },
};

impl ErrorConverter {
  pub fn operator_call_13(&self, e: &CannotCallNonFunction) -> String {
    let t = follow_type_id(e.ty);

    if let Some(union_ty) = get_type_id::<UnionType>(t) {
      let mut err = String::from("Cannot call a value of the union type:");

      for option in &union_ty.options {
        let option = follow_type_id(*option);

        if get_type_id::<FunctionType>(option).is_some()
          || self.find_call_metamethod(option).is_some()
        {
          err.push_str("\n  | ");
          err.push_str(&to_string_type_id(option));
          continue;
        }

        return format!(
          "Cannot call a value of type {} in union:\n  {}",
          to_string_type_id(option),
          to_string_type_id(e.ty)
        );
      }

      err.push_str("\nWe are unable to determine the appropriate result type for such a call.");
      return err;
    }

    if let Some(primitive_ty) = get_type_id::<PrimitiveType>(t)
      && primitive_ty.r#type == PrimitiveType::FUNCTION
    {
      return format!(
        "The type {} is not precise enough for us to determine the appropriate result type of this call.",
        to_string_type_id(e.ty)
      );
    }

    format!("Cannot call a value of type {}", to_string_type_id(e.ty))
  }
}
