use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{
    error_converter::ErrorConverter,
    type_instantiation_count_mismatch::TypeInstantiationCountMismatch,
  },
};

impl ErrorConverter {
  pub fn operator_call_53(&self, e: &TypeInstantiationCountMismatch) -> String {
    LUAU_ASSERT!(
      e.provided_types() > e.maximum_types() || e.provided_type_packs() > e.maximum_type_packs()
    );

    let mut result = String::from("Too many type parameters passed to ");

    if let Some(function_name) = e.function_name() {
      result.push('\'');
      result.push_str(function_name);
      result.push_str("', which is typed as ");
    } else {
      result.push_str("function typed as ");
    }

    result.push_str(&to_string_type_id(e.function_type()));

    result.push_str(". Expected ");

    if e.provided_types() > e.maximum_types() {
      result.push_str("at most ");
      result.push_str(&e.maximum_types().to_string());
      result.push_str(" type parameter");
      if e.maximum_types() != 1 {
        result.push('s');
      }
      result.push_str(", but ");
      result.push_str(&e.provided_types().to_string());
      result.push_str(" provided");

      if e.provided_type_packs() > e.maximum_type_packs() {
        result.push_str(". Also expected ");
      }
    }

    if e.provided_type_packs() > e.maximum_type_packs() {
      result.push_str("at most ");
      result.push_str(&e.maximum_type_packs().to_string());
      result.push_str(" type pack");
      if e.maximum_type_packs() != 1 {
        result.push('s');
      }
      result.push_str(", but ");
      result.push_str(&e.provided_type_packs().to_string());
      result.push_str(" provided");
    }

    result.push('.');
    result
  }
}
