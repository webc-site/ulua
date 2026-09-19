use alloc::string::String;
use core::fmt::Write;

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

    // 复数后缀：数量为 1 时无 s，与 C++ 输出逐字节一致
    let plural = |n: usize| if n == 1 { "" } else { "s" };

    let mut result = String::from("Too many type parameters passed to ");

    if let Some(function_name) = e.function_name() {
      let _ = write!(result, "'{function_name}', which is typed as ");
    } else {
      result.push_str("function typed as ");
    }

    let _ = write!(
      result,
      "{}. Expected ",
      to_string_type_id(e.function_type())
    );

    if e.provided_types() > e.maximum_types() {
      let _ = write!(
        result,
        "at most {} type parameter{}, but {} provided",
        e.maximum_types(),
        plural(e.maximum_types()),
        e.provided_types()
      );

      if e.provided_type_packs() > e.maximum_type_packs() {
        result.push_str(". Also expected ");
      }
    }

    if e.provided_type_packs() > e.maximum_type_packs() {
      let _ = write!(
        result,
        "at most {} type pack{}, but {} provided",
        e.maximum_type_packs(),
        plural(e.maximum_type_packs()),
        e.provided_type_packs()
      );
    }

    result.push('.');
    result
  }
}
