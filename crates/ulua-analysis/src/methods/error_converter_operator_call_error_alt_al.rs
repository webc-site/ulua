use alloc::string::String;
use core::fmt::Write;

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
  type_aliases::{error_type::ErrorType, type_id::TypeId, type_pack_id::TypePackId},
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

    // "types 用逗号连接；pack types 再以 ", " 前缀追加"，与 C++ 输出逐字节一致
    let append_argument_list =
      |result: &mut String, type_arguments: &[TypeId], pack_arguments: &[TypePackId]| {
        for (i, arg) in type_arguments.iter().enumerate() {
          if i > 0 {
            result.push_str(", ");
          }
          let _ = write!(result, "{}", to_string_type_id(*arg));
        }
        for pack_arg in pack_arguments.iter() {
          let _ = write!(result, ", {}", to_string_type_pack_id(*pack_arg));
        }
      };

    // unary operators
    if let Some(unary_string) = find_unary_op(function_name) {
      let mut result = format!("Operator '{unary_string}' could not be applied to ");

      if tfit_ref.type_arguments.len() == 1 && tfit_ref.pack_arguments.is_empty() {
        let _ = write!(
          result,
          "operand of type {}",
          to_string_type_id(tfit_ref.type_arguments[0])
        );

        if function_name != "not" {
          let _ = write!(
            result,
            "; there is no corresponding overload for __{function_name}"
          );
        }
      } else {
        result.push_str("operands of types ");
        append_argument_list(
          &mut result,
          &tfit_ref.type_arguments,
          &tfit_ref.pack_arguments,
        );
      }

      return result;
    }

    // binary operators
    if let Some(binary_string) = find_binary_op(function_name) {
      let mut result =
        format!("Operator '{binary_string}' could not be applied to operands of types ");

      if tfit_ref.type_arguments.len() == 2 && tfit_ref.pack_arguments.is_empty() {
        let _ = write!(
          result,
          "{} and {}",
          to_string_type_id(tfit_ref.type_arguments[0]),
          to_string_type_id(tfit_ref.type_arguments[1])
        );
      } else {
        append_argument_list(
          &mut result,
          &tfit_ref.type_arguments,
          &tfit_ref.pack_arguments,
        );
      }

      let _ = write!(
        result,
        "; there is no corresponding overload for __{function_name}"
      );

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
  .find_map(|(key, value)| (*key == name).then_some(*value))
}

fn find_unary_op(name: &str) -> Option<&'static str> {
  [("unm", "-"), ("len", "#"), ("not", "not")]
    .iter()
    .find_map(|(key, value)| (*key == name).then_some(*value))
}

fn is_unreachable_type_function(name: &str) -> bool {
  ["refine", "singleton", "union", "intersect", "and", "or"].contains(&name)
}
