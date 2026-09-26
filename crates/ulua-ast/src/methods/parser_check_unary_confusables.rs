use crate::{
  enums::type_lexer::Type,
  records::{ast_expr_unary::AstExprUnaryOp, parser::Parser},
};

impl Parser {
  pub fn check_unary_confusables(&mut self) -> Option<AstExprUnaryOp> {
    let curr = *self.lexer.current();

    // early-out: need to check if this is a possible confusable quickly
    if curr.r#type != Type::BANG {
      return None;
    }

    // slow path: possible confusable
    // 早退已保证 type == '!'，C++ 原文其内的二次同值判断为恒真死分支，删除。
    self.report(
      curr.location,
      format_args!("Unexpected '!'; did you mean 'not'?"),
    );
    Some(AstExprUnaryOp::Not)
  }
}
