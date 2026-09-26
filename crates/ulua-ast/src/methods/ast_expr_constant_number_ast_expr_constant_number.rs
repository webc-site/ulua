use crate::{
  enums::constant_number_parse_result::ConstantNumberParseResult,
  records::{
    ast_expr::AstExpr, ast_expr_constant_number::AstExprConstantNumber, location::Location,
  },
};

impl_ast_node_new!(
  AstExprConstantNumber,
  AstExpr,
  location: Location,
  value: f64,
  parse_result: ConstantNumberParseResult,
);
