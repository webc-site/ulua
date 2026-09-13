use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    get_type_alt_j::get_type_id, to_string_to_string_alt_c::to_string_type_id,
    to_string_to_string_alt_d::to_string_type_pack_id,
  },
  records::{
    error_converter::ErrorConverter, type_function_instance_type::TypeFunctionInstanceType,
    uninhabited_type_function::UninhabitedTypeFunction,
  },
  type_aliases::error_type::ErrorType,
};
impl ErrorConverter {
  pub fn operator_call_60(&self, e: &UninhabitedTypeFunction) -> String {
    let Some(tfit_ref) = get_type_id::<TypeFunctionInstanceType>(e.ty) else {
      LUAU_ASSERT!(false);
      return format!(
        "Internal error: Unexpected type {} flagged as an uninhabited type function.",
        to_string_type_id(e.ty)
      );
    };

    // SAFETY: function 指向 TypeFunctionInstance 内部的存活 TypedFunction。
    let function_name = unsafe { &(*tfit_ref.function.as_ptr()).name };

    // unary operators
    if let Some(unary_string) = find_unary_op(function_name) {
      let mut result = format!("Operator '{}' could not be applied to ", unary_string);

      if tfit_ref.type_arguments.len() == 1 && tfit_ref.pack_arguments.is_empty() {
        result.push_str(&format!(
          "operand of type {}",
          to_string_type_id(tfit_ref.type_arguments[0])
        ));

        if function_name != "not" {
          result.push_str(&format!(
            "; there is no corresponding overload for __{}",
            function_name
          ));
        }
      } else {
        result.push_str("operands of types ");

        let mut is_first = true;
        for arg in &tfit_ref.type_arguments {
          if !is_first {
            result.push_str(", ");
          }
          result.push_str(&to_string_type_id(*arg));
          is_first = false;
        }

        for pack_arg in &tfit_ref.pack_arguments {
          result.push_str(&format!(", {}", to_string_type_pack_id(*pack_arg)));
        }
      }

      return result;
    }

    // binary operators
    if let Some(binary_string) = find_binary_op(function_name) {
      let mut result = format!(
        "Operator '{}' could not be applied to operands of types ",
        binary_string
      );

      if tfit_ref.type_arguments.len() == 2 && tfit_ref.pack_arguments.is_empty() {
        result.push_str(&format!(
          "{} and {}",
          to_string_type_id(tfit_ref.type_arguments[0]),
          to_string_type_id(tfit_ref.type_arguments[1])
        ));
      } else {
        let mut is_first = true;
        for arg in &tfit_ref.type_arguments {
          if !is_first {
            result.push_str(", ");
          }
          result.push_str(&to_string_type_id(*arg));
          is_first = false;
        }

        for pack_arg in &tfit_ref.pack_arguments {
          result.push_str(&format!(", {}", to_string_type_pack_id(*pack_arg)));
        }
      }

      result.push_str(&format!(
        "; there is no corresponding overload for __{}",
        function_name
      ));

      return result;
    }

    // miscellaneous
    if function_name == "keyof" || function_name == "rawkeyof" {
      if tfit_ref.type_arguments.len() == 1 && tfit_ref.pack_arguments.is_empty() {
        return format!(
          "Type '{}' does not have keys, so '{}' is invalid",
          to_string_type_id(tfit_ref.type_arguments[0]),
          to_string_type_id(e.ty)
        );
      } else {
        return format!(
          "Type function instance {} is ill-formed, and thus invalid",
          to_string_type_id(e.ty)
        );
      }
    }

    if function_name == "index" || function_name == "rawget" {
      if tfit_ref.type_arguments.len() != 2 {
        return format!(
          "Type function instance {} is ill-formed, and thus invalid",
          to_string_type_id(e.ty)
        );
      }

      let second_arg = tfit_ref.type_arguments[1];
      if get_type_id::<ErrorType>(second_arg).is_some() {
        return format!(
          "Second argument to {}<{}, _> is not a valid index type",
          function_name,
          to_string_type_id(tfit_ref.type_arguments[0])
        );
      } else {
        return format!(
          "Property '{}' does not exist on type '{}'",
          to_string_type_id(tfit_ref.type_arguments[1]),
          to_string_type_id(tfit_ref.type_arguments[0])
        );
      }
    }

    if is_unreachable_type_function(function_name) {
      return format!(
        "Type function instance {} is uninhabited\nThis is likely to be a bug, please report it at https://github.com/luau-lang/luau/issues",
        to_string_type_id(e.ty)
      );
    }

    // Everything should be specialized above to report a more descriptive error that hopefully does not mention "type functions" explicitly.
    // If we produce this message, it's an indication that we've missed a specialization and it should be fixed!
    format!(
      "Type function instance {} is uninhabited",
      to_string_type_id(e.ty)
    )
  }
}

fn find_binary_op(name: &str) -> Option<&'static str> {
  [
    ("add", "+"),
    ("sub", "-"),
    ("mul", "*"),
    ("div", "/"),
    ("idiv", "//"),
    ("pow", "^"),
    ("mod", "%"),
    ("concat", ".."),
    ("lt", "< or >="),
    ("le", "<= or >"),
    ("eq", "== or ~="),
  ]
  .iter()
  .find_map(|(key, value)| if *key == name { Some(*value) } else { None })
}

fn find_unary_op(name: &str) -> Option<&'static str> {
  [("unm", "-"), ("len", "#"), ("not", "not")]
    .iter()
    .find_map(|(key, value)| if *key == name { Some(*value) } else { None })
}

fn is_unreachable_type_function(name: &str) -> bool {
  ["refine", "singleton", "union", "intersect", "and", "or"].contains(&name)
}
