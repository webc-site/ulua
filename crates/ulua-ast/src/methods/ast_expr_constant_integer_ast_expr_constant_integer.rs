use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::{
    ast_expr::AstExpr, ast_expr_constant_integer::AstExprConstantInteger, location::Location,
  },
};

impl_ast_node_new!(
  AstExprConstantInteger,
  AstExpr,
  location: Location,
  value: i64,
  parse_result: ConstantNumberParseResult,
);
