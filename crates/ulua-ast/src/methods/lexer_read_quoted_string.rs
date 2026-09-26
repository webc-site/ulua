//! `Lexeme Lexer::read_quoted_string()` — Ast/src/Lexer.cpp:583.

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  records::{lexeme::Lexeme, lexer::Lexer, location::Location},
};

impl Lexer {
  pub(crate) fn read_quoted_string(&mut self) -> Lexeme {
    let start = self.position();

    let delimiter = self.peekch();
    LUAU_ASSERT!(delimiter == '\'' || delimiter == '"');
    self.consume();

    let start_offset = self.offset;

    while self.peekch() != delimiter {
      match self.peekch() {
        '\0' | '\r' | '\n' => {
          return Lexeme::new(Location::new(start, self.position()), Type::BROKEN_STRING);
        }
        '\\' => self.read_backslash_in_string(),
        _ => self.consume(),
      }
    }

    self.consume();

    Lexeme::with_data(
      Location::new(start, self.position()),
      Type::QUOTED_STRING,
      // ⇔ cpp `Lexeme(..., &buffer[startOffset], offset - startOffset - 1)`（Lexer.cpp:613）：
      // start_offset 在消费开引号后记录，载荷不含闭引号、但闭引号字节仍界内
      // （get_quote_style 依赖），成功路径上界 offset-1 恒在缓冲界内。
      &self.buffer[start_offset as usize..(self.offset - 1) as usize],
    )
  }
}
