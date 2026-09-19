use crate::records::{
  ast_expr_binary::AstExprBinaryOp,
  lexeme::{Lexeme, Type},
  parser::Parser,
};

impl Parser {
  pub(crate) fn parse_compound_op(&self, l: &Lexeme) -> Option<AstExprBinaryOp> {
    if l.r#type == Type::ADD_ASSIGN {
      Some(AstExprBinaryOp::Add)
    } else if l.r#type == Type::SUB_ASSIGN {
      Some(AstExprBinaryOp::Sub)
    } else if l.r#type == Type::MUL_ASSIGN {
      Some(AstExprBinaryOp::Mul)
    } else if l.r#type == Type::DIV_ASSIGN {
      Some(AstExprBinaryOp::Div)
    } else if l.r#type == Type::FLOOR_DIV_ASSIGN {
      Some(AstExprBinaryOp::FloorDiv)
    } else if l.r#type == Type::MOD_ASSIGN {
      Some(AstExprBinaryOp::Mod)
    } else if l.r#type == Type::POW_ASSIGN {
      Some(AstExprBinaryOp::Pow)
    } else if l.r#type == Type::CONCAT_ASSIGN {
      Some(AstExprBinaryOp::Concat)
    } else {
      None
    }
  }
}
