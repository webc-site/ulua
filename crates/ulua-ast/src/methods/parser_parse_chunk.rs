use crate::{
  enums::type_lexer::Type,
  records::{ast_stat_block::AstStatBlock, parser::Parser},
};

impl Parser {
  pub fn parse_chunk(&mut self) -> *mut AstStatBlock {
    let result = self.parse_block();

    if self.lexer.current().r#type != Type::EOF {
      self.expect_and_consume_fail(Type::EOF, "");
    }

    result
  }
}
