use crate::records::{
  ast_expr_unary::AstExprUnaryOp,
  lexeme::{Lexeme, Type},
  parser::Parser,
};

impl Parser {
  pub(crate) fn parse_unary_op(&self, l: &Lexeme) -> Option<AstExprUnaryOp> {
    if l.r#type == Type::RESERVED_NOT {
      Some(AstExprUnaryOp::Not)
    } else if l.r#type == Type('-' as i32) {
      Some(AstExprUnaryOp::Minus)
    } else if l.r#type == Type('#' as i32) {
      Some(AstExprUnaryOp::Len)
    } else {
      None
    }
  }
}

pub fn parser_parse_unary_op(this: &Parser, l: &Lexeme) -> Option<AstExprUnaryOp> {
  this.parse_unary_op(l)
}
