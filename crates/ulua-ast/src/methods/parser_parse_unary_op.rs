use crate::{
  enums::type_lexer::Type,
  records::{ast_expr_unary::AstExprUnaryOp, lexeme::Lexeme, parser::Parser},
};

// 与 parse_binary_op 同构：Type 派生结构相等，关联常量直接作 match 模式，
// 编译期派发取代 3 段 if-else 链。
const T_SUB: Type = Type(b'-' as i32);
const T_LEN: Type = Type(b'#' as i32);

impl Parser {
  pub(crate) fn parse_unary_op(&self, l: &Lexeme) -> Option<AstExprUnaryOp> {
    match l.r#type {
      Type::RESERVED_NOT => Some(AstExprUnaryOp::Not),
      T_SUB => Some(AstExprUnaryOp::Minus),
      T_LEN => Some(AstExprUnaryOp::Len),
      _ => None,
    }
  }
}
