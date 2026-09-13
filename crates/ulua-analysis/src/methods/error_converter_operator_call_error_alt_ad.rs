use alloc::string::String;

use ulua_ast::functions::to_string_ast_alt_b::to_string_ast_expr_binary_op;

use crate::{
  enums::op_kind::OpKind,
  records::{
    cannot_infer_binary_operation::CannotInferBinaryOperation, error_converter::ErrorConverter,
  },
};

impl ErrorConverter {
  pub fn operator_call_16(&self, e: &CannotInferBinaryOperation) -> String {
    let mut result = String::from("Unknown type used in ");
    result.push_str(&to_string_ast_expr_binary_op(e.op()));

    match e.kind() {
      OpKind::Comparison => {
        result.push_str(" comparison");
      }
      OpKind::Operation => {
        result.push_str(" operation");
      }
    }

    if let Some(suggested) = e.suggested_to_annotate() {
      result.push_str("; consider adding a type annotation to '");
      result.push_str(suggested);
      result.push('\'');
    }

    result
  }
}
