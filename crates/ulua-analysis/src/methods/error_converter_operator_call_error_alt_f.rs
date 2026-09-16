use alloc::string::String;

use ulua_ast::functions::to_string_ast_alt_b::to_string_ast_expr_binary_op;

use crate::{
  functions::to_string_to_string_alt_c::to_string_type_id,
  records::{
    cannot_compare_unrelated_types::CannotCompareUnrelatedTypes, error_converter::ErrorConverter,
  },
};

impl ErrorConverter {
  pub fn operator_call_14(&self, e: &CannotCompareUnrelatedTypes) -> String {
    let left_str = to_string_type_id(e.left);
    let right_str = to_string_type_id(e.right);
    let op_str = to_string_ast_expr_binary_op(e.op);
    format!(
      "Cannot compare unrelated types '{}' and '{}' with '{}'",
      left_str, right_str, op_str
    )
  }
}
