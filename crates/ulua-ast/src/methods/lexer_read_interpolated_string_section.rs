//! `Lexeme Lexer::read_interpolated_string_section(Position start, Lexeme::Type formatType, Lexeme::Type endType)`
//! — Ast/src/Lexer.cpp:626.

use crate::{
  enums::{brace_type::BraceType, type_lexer::Type},
  records::{lexeme::Lexeme, lexer::Lexer, location::Location, position::Position},
};

impl Lexer {
  pub(crate) fn read_interpolated_string_section(
    &mut self,
    start: Position,
    format_type: Type,
    end_type: Type,
  ) -> Lexeme {
    let start_offset = self.offset;

    while self.peekch() != '`' {
      match self.peekch() {
        '\0' | '\r' | '\n' => {
          return Lexeme::new(Location::new(start, self.position()), Type::BROKEN_STRING);
        }

        '\\' => {
          // Allow for \u{}, which would otherwise be consumed by looking for {
          if self.peekch_ahead(1) == 'u' && self.peekch_ahead(2) == '{' {
            self.consume(); // backslash
            self.consume(); // u
            self.consume(); // {
          } else {
            self.read_backslash_in_string();
          }
        }

        '{' => {
          self.brace_stack.push(BraceType::InterpolatedString);

          if self.peekch_ahead(1) == '{' {
            let broken_double_brace = Lexeme::with_data(
              Location::new(start, self.position()),
              Type::BROKEN_INTERP_DOUBLE_BRACE,
              // ⇔ cpp `Lexeme(..., &buffer[startOffset], offset - startOffset)`
              // （Lexer.cpp:659）：载荷与源缓冲同寿命。
              &self.buffer[start_offset as usize..self.offset as usize],
            );
            self.consume();
            self.consume();
            return broken_double_brace;
          }

          self.consume();
          return Lexeme::with_data(
            Location::new(start, self.position()),
            format_type,
            // ⇔ cpp `Lexeme(..., &buffer[startOffset], offset - startOffset - 1)`
            // （Lexer.cpp:666；已 consume 起始 `{`，载荷不含它），载荷与源缓冲同寿命。
            &self.buffer[start_offset as usize..(self.offset - 1) as usize],
          );
        }

        _ => self.consume(),
      }
    }

    self.consume();

    Lexeme::with_data(
      Location::new(start, self.position()),
      end_type,
      // ⇔ cpp `Lexeme(..., &buffer[startOffset], offset - startOffset - 1)`
      // （read_interpolated_string_section 尾；已 consume 闭 backquote，载荷不含它），
      // 载荷与源缓冲同寿命。
      &self.buffer[start_offset as usize..(self.offset - 1) as usize],
    )
  }
}
