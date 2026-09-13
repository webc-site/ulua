use crate::{
  enums::separator::Separator,
  records::{lexeme::Type, parser::Parser},
};

impl Parser {
  pub fn table_separator(&mut self) -> Separator {
    if self.lexer.current().r#type == Type(',' as i32) {
      Separator::Comma
    } else if self.lexer.current().r#type == Type(';' as i32) {
      Separator::Semicolon
    } else {
      Separator::Missing
    }
  }
}

pub fn parser_table_separator(this: &mut Parser) -> Separator {
  this.table_separator()
}
