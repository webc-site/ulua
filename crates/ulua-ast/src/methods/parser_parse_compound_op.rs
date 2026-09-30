use crate::{
  enums::type_lexer::Type,
  records::{ast_expr_binary::AstExprBinaryOp, lexeme::Lexeme, parser::Parser},
};

impl Parser {
  /// 复合赋值运算符：对常量 `Type` 的单值 match 派发（跳转表），替代 else-if 链
  pub(crate) fn parse_compound_op(&self, l: &Lexeme) -> Option<AstExprBinaryOp> {
    match l.r#type {
      Type::ADD_ASSIGN => Some(AstExprBinaryOp::Add),
      Type::SUB_ASSIGN => Some(AstExprBinaryOp::Sub),
      Type::MUL_ASSIGN => Some(AstExprBinaryOp::Mul),
      Type::DIV_ASSIGN => Some(AstExprBinaryOp::Div),
      Type::FLOOR_DIV_ASSIGN => Some(AstExprBinaryOp::FloorDiv),
      Type::MOD_ASSIGN => Some(AstExprBinaryOp::Mod),
      Type::POW_ASSIGN => Some(AstExprBinaryOp::Pow),
      Type::CONCAT_ASSIGN => Some(AstExprBinaryOp::Concat),
      _ => None,
    }
  }
}
