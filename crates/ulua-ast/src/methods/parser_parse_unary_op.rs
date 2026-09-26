use crate::{
  enums::type_lexer::Type,
  records::{ast_expr_unary::AstExprUnaryOp, lexeme::Lexeme, parser::Parser},
};

impl Parser {
  pub(crate) fn parse_unary_op(&self, l: &Lexeme) -> Option<AstExprUnaryOp> {
    match l.r#type {
      Type::RESERVED_NOT => Some(AstExprUnaryOp::Not),
      Type::MINUS => Some(AstExprUnaryOp::Minus),
      Type::HASH => Some(AstExprUnaryOp::Len),
      _ => None,
    }
  }
}
