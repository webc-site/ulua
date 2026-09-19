//! `Lexer::read_symbol_pair` — `read_next` 中九处「吃首字符 → 看下一字符决定
//! 双字符/单字符符号」重复模式的收口（`= ==`、`< <=`、`+ +=`、`: ::` 等）。

use crate::records::{
  lexeme::{Lexeme, Type},
  lexer::Lexer,
  location::Location,
  position::Position,
};

impl Lexer {
  /// 消费首字符 `first`；若紧随字符为 `second` 则一并消费并返回双字符
  /// 词素（类型 `paired`），否则回落为 `first` 的单字符词素。
  #[inline]
  pub(crate) fn read_symbol_pair(
    &mut self,
    start: Position,
    first: char,
    second: char,
    paired: Type,
  ) -> Lexeme {
    self.consume();
    if self.peekch() == second {
      self.consume();
      Lexeme::new(Location::with_length(start, 2), paired)
    } else {
      Lexeme::from_char(Location::with_length(start, 1), first)
    }
  }
}
