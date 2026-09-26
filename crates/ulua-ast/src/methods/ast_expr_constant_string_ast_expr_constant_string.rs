use crate::{
  enums::quote_style_ast::QuoteStyle,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    location::Location,
  },
};

impl_ast_node_new!(
  AstExprConstantString,
  AstExpr,
  location: Location,
  value: AstArray<u8>,
  quote_style: QuoteStyle,
);
