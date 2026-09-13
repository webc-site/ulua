use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::{
    ast_expr::AstExpr, ast_expr_constant_integer::AstExprConstantInteger, ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprConstantInteger {
  pub fn new(location: Location, value: i64, parse_result: ConstantNumberParseResult) -> Self {
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

pub fn ast_expr_constant_integer_ast_expr_constant_integer(
  location: Location,
  value: i64,
  parse_result: ConstantNumberParseResult,
) -> AstExprConstantInteger {
  AstExprConstantInteger::new(location, value, parse_result)
}
