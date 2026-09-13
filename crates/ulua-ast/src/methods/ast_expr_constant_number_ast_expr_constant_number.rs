use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::{
    ast_expr::AstExpr, ast_expr_constant_number::AstExprConstantNumber, ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprConstantNumber {
  pub fn new(location: Location, value: f64, parse_result: ConstantNumberParseResult) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      value,
      parse_result,
    }
  }
}

pub fn ast_expr_constant_number_ast_expr_constant_number(
  location: Location,
  value: f64,
  parse_result: ConstantNumberParseResult,
) -> AstExprConstantNumber {
  AstExprConstantNumber::new(location, value, parse_result)
}
