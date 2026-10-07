//! `Lexeme Lexer::read_number(const Position& start, unsigned int startOffset)`
//! — Ast/src/Lexer.cpp:679.

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  functions::char_classifier::{is_digit, is_identifier_char},
  records::{lexeme::Lexeme, lexer::Lexer, location::Location, position::Position},
};

impl Lexer {
  pub(crate) fn read_number(&mut self, start: &Position, start_offset: u32) -> Lexeme {
    LUAU_ASSERT!(is_digit(self.peekch()));

    // This function does not do the number parsing - it only skips a
    // number-like pattern. The resulting string is later converted to a
    // number with proper verification.
    //
    // C++ do-while 直译：无条件消费首字符（前置 assert 已保证是数字），
    // 再按条件继续；条件内 peekch 单次求值，取代逐分支重复取值。
    self.consume();
    while {
      let ch = self.peekch();
      is_digit(ch) || ch == '.' || ch == '_'
    } {
      self.consume();
    }

    if self.peekch() == 'e' || self.peekch() == 'E' {
      self.consume();

      if self.peekch() == '+' || self.peekch() == '-' {
        self.consume();
      }
    }

    // cpp `isAlpha(ch) || isDigit(ch) || ch == '_'`（Lexer.cpp:699）——数字字面量
    // 尾部（`0xff`、`123i` 等）继续按标识符字符吞掉，判定单源于 char_classifier。
    while is_identifier_char(self.peekch()) {
      self.consume();
    }

    Lexeme::with_data(
      Location::new(*start, self.position()),
      Type::NUMBER,
      // ⇔ cpp `Lexeme(..., &buffer[startOffset], offset - startOffset)`（readNumber
      // 尾）：start_offset 为进入数字扫描前的缓冲内偏移，循环仅 consume 前进，
      // [start_offset, offset) 恒在源缓冲界内；载荷与源缓冲同寿命。
      &self.buffer[start_offset as usize..self.offset as usize],
    )
  }
}
