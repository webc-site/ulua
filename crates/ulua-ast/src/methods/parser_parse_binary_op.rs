use crate::{
  enums::type_lexer::Type,
  records::{ast_expr_binary::AstExprBinaryOp as Op, lexeme::Lexeme, parser::Parser},
};

impl Parser {
  pub(crate) fn parse_binary_op(&self, l: &Lexeme) -> Option<Op> {
    // 原 17 段 if-else 链改 match：编译器按比较树/跳转表派发。
    match l.r#type {
      Type::PLUS => Some(Op::Add),
      Type::MINUS => Some(Op::Sub),
      Type::STAR => Some(Op::Mul),
      Type::SLASH => Some(Op::Div),
      Type::FLOOR_DIV => Some(Op::FloorDiv),
      Type::PERCENT => Some(Op::Mod),
      Type::CARET => Some(Op::Pow),
      Type::DOT2 => Some(Op::Concat),
      Type::NOT_EQUAL => Some(Op::CompareNe),
      Type::EQUAL => Some(Op::CompareEq),
      Type::LESS => Some(Op::CompareLt),
      Type::LESS_EQUAL => Some(Op::CompareLe),
      Type::GREATER => Some(Op::CompareGt),
      Type::GREATER_EQUAL => Some(Op::CompareGe),
      Type::RESERVED_AND => Some(Op::And),
      Type::RESERVED_OR => Some(Op::Or),
      _ => None,
    }
  }
}
