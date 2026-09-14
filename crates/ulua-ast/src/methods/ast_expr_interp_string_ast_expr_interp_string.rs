use core::ffi::c_char;

use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString,
    ast_node::AstNode, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprInterpString {
  pub fn new(
    location: Location,
    strings: AstArray<AstArray<c_char>>,
    expressions: AstArray<*mut AstExpr>,
  ) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      strings,
      expressions,
    }
  }
}

pub fn ast_expr_interp_string_ast_expr_interp_string(
  location: Location,
  strings: AstArray<AstArray<c_char>>,
  expressions: AstArray<*mut AstExpr>,
) -> AstExprInterpString {
  AstExprInterpString::new(location, strings, expressions)
}
