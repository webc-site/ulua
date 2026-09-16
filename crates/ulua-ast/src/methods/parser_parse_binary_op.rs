use crate::records::{
  ast_expr_binary::AstExprBinaryOp as Op,
  lexeme::{Lexeme, Type},
  parser::Parser,
};

impl Parser {
  pub(crate) fn parse_binary_op(&self, l: &Lexeme) -> Option<Op> {
    if l.r#type == Type('+' as i32) {
      Some(Op::Add)
    } else if l.r#type == Type('-' as i32) {
      Some(Op::Sub)
    } else if l.r#type == Type('*' as i32) {
      Some(Op::Mul)
    } else if l.r#type == Type('/' as i32) {
      Some(Op::Div)
    } else if l.r#type == Type::FLOOR_DIV {
      Some(Op::FloorDiv)
    } else if l.r#type == Type('%' as i32) {
      Some(Op::Mod)
    } else if l.r#type == Type('^' as i32) {
      Some(Op::Pow)
    } else if l.r#type == Type::DOT2 {
      Some(Op::Concat)
    } else if l.r#type == Type::NOT_EQUAL {
      Some(Op::CompareNe)
    } else if l.r#type == Type::EQUAL {
      Some(Op::CompareEq)
    } else if l.r#type == Type('<' as i32) {
      Some(Op::CompareLt)
    } else if l.r#type == Type::LESS_EQUAL {
      Some(Op::CompareLe)
    } else if l.r#type == Type('>' as i32) {
      Some(Op::CompareGt)
    } else if l.r#type == Type::GREATER_EQUAL {
      Some(Op::CompareGe)
    } else if l.r#type == Type::RESERVED_AND {
      Some(Op::And)
    } else if l.r#type == Type::RESERVED_OR {
      Some(Op::Or)
    } else {
      None
    }
  }
}

pub fn parser_parse_binary_op(this: &Parser, l: &Lexeme) -> Option<Op> {
  this.parse_binary_op(l)
}
