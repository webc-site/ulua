use crate::{
  enums::type_lexer::Type,
  records::{ast_expr_binary::AstExprBinaryOp as Op, lexeme::Lexeme, parser::Parser},
};

// 单字符 token 型即字节码值；Type 派生结构相等，常量可直接作 match 模式臂。
const T_ADD: Type = Type(b'+' as i32);
const T_SUB: Type = Type(b'-' as i32);
const T_MUL: Type = Type(b'*' as i32);
const T_DIV: Type = Type(b'/' as i32);
const T_MOD: Type = Type(b'%' as i32);
const T_POW: Type = Type(b'^' as i32);
const T_LT: Type = Type(b'<' as i32);
const T_GT: Type = Type(b'>' as i32);

impl Parser {
  pub(crate) fn parse_binary_op(&self, l: &Lexeme) -> Option<Op> {
    // 原 17 段 if-else 链改 match：编译器按比较树/跳转表派发。
    match l.r#type {
      T_ADD => Some(Op::Add),
      T_SUB => Some(Op::Sub),
      T_MUL => Some(Op::Mul),
      T_DIV => Some(Op::Div),
      Type::FLOOR_DIV => Some(Op::FloorDiv),
      T_MOD => Some(Op::Mod),
      T_POW => Some(Op::Pow),
      Type::DOT2 => Some(Op::Concat),
      Type::NOT_EQUAL => Some(Op::CompareNe),
      Type::EQUAL => Some(Op::CompareEq),
      T_LT => Some(Op::CompareLt),
      Type::LESS_EQUAL => Some(Op::CompareLe),
      T_GT => Some(Op::CompareGt),
      Type::GREATER_EQUAL => Some(Op::CompareGe),
      Type::RESERVED_AND => Some(Op::And),
      Type::RESERVED_OR => Some(Op::Or),
      _ => None,
    }
  }
}
