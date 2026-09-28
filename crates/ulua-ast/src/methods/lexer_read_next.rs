//! `Lexeme Lexer::read_next()` — Ast/src/Lexer.cpp:719. The single-token dispatch.

use crate::{
  enums::{brace_type::BraceType, type_lexer::Type},
  functions::char_classifier::{is_digit, is_identifier_start_char},
  records::{ast_name::AstName, lexeme::Lexeme, lexer::Lexer, location::Location},
};

/// 空属性名（cpp `""` 字面量）：静态 NUL 结尾空字节串构造的非驻留 [`AstName`]，
/// 保持「非空指针的空名」语义与驻留空名一致（区别于 null 名）。
/// `const`（每使用点按值复制、指针指向提升为 'static 的字节字面量）而非 `static`：
/// `AstName` 含裸指针不满足 `Sync`。
const EMPTY_ATTRIBUTE_NAME: AstName = AstName::from_static(b"");

impl Lexer {
  pub(crate) fn read_next(&mut self) -> Lexeme {
    let start = self.position();

    match self.peekch() {
      '\0' => Lexeme::new(Location::with_length(start, 0), Type::EOF),

      '-' => {
        if self.peekch_ahead(1) == '>' {
          self.consume();
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::SKINNY_ARROW)
        } else if self.peekch_ahead(1) == '=' {
          self.consume();
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::SUB_ASSIGN)
        } else if self.peekch_ahead(1) == '-' {
          self.read_comment_body()
        } else {
          self.consume();
          Lexeme::from_char(Location::with_length(start, 1), '-')
        }
      }

      '[' => {
        let sep = self.skip_long_separator();

        if sep >= 0 {
          self.read_long_string(&start, sep, Type::RAW_STRING, Type::BROKEN_STRING)
        } else if sep == -1 {
          Lexeme::from_char(Location::with_length(start, 1), '[')
        } else {
          Lexeme::new(Location::new(start, self.position()), Type::BROKEN_STRING)
        }
      }

      '{' => {
        self.consume();

        if !self.brace_stack.is_empty() {
          self.brace_stack.push(BraceType::Normal);
        }

        Lexeme::from_char(Location::with_length(start, 1), '{')
      }

      '}' => {
        self.consume();

        // pop 一次同时完成判空与取顶，取代 is_empty + last().unwrap() + pop
        // 的三次操作（并消掉 unwrap）。
        let Some(brace_stack_top) = self.brace_stack.pop() else {
          return Lexeme::from_char(Location::with_length(start, 1), '}');
        };

        if brace_stack_top != BraceType::InterpolatedString {
          return Lexeme::from_char(Location::with_length(start, 1), '}');
        }

        self.read_interpolated_string_section(
          start,
          Type::INTERP_STRING_MID,
          Type::INTERP_STRING_END,
        )
      }

      '=' => self.read_symbol_pair(start, '=', '=', Type::EQUAL),

      '<' => self.read_symbol_pair(start, '<', '=', Type::LESS_EQUAL),

      '>' => self.read_symbol_pair(start, '>', '=', Type::GREATER_EQUAL),

      '~' => self.read_symbol_pair(start, '~', '=', Type::NOT_EQUAL),

      '"' | '\'' => self.read_quoted_string(),

      '`' => self.read_interpolated_string_begin(),

      '.' => {
        self.consume();

        if self.peekch() == '.' {
          self.consume();

          if self.peekch() == '.' {
            self.consume();
            Lexeme::new(Location::with_length(start, 3), Type::DOT3)
          } else if self.peekch() == '=' {
            self.consume();
            Lexeme::new(Location::with_length(start, 3), Type::CONCAT_ASSIGN)
          } else {
            Lexeme::new(Location::with_length(start, 2), Type::DOT2)
          }
        } else if is_digit(self.peekch()) {
          self.read_number(&start, self.offset - 1)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '.')
        }
      }

      '+' => self.read_symbol_pair(start, '+', '=', Type::ADD_ASSIGN),

      '/' => {
        self.consume();

        let ch = self.peekch();

        if ch == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::DIV_ASSIGN)
        } else if ch == '/' {
          self.consume();

          if self.peekch() == '=' {
            self.consume();
            Lexeme::new(Location::with_length(start, 3), Type::FLOOR_DIV_ASSIGN)
          } else {
            Lexeme::new(Location::with_length(start, 2), Type::FLOOR_DIV)
          }
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '/')
        }
      }

      '*' => self.read_symbol_pair(start, '*', '=', Type::MUL_ASSIGN),

      '%' => self.read_symbol_pair(start, '%', '=', Type::MOD_ASSIGN),

      '^' => self.read_symbol_pair(start, '^', '=', Type::POW_ASSIGN),

      ':' => self.read_symbol_pair(start, ':', ':', Type::DOUBLE_COLON),

      '(' | ')' | ']' | ';' | ',' | '#' | '?' | '&' | '|' => {
        let ch = self.peekch();
        self.consume();

        Lexeme::from_char(Location::with_length(start, 1), ch)
      }

      '@' => {
        if self.peekch_ahead(1) == '[' {
          self.consume();
          self.consume();

          Lexeme::new(Location::with_length(start, 2), Type::ATTRIBUTE_OPEN)
        } else {
          // consume @ first
          self.consume();

          if is_identifier_start_char(self.peekch()) {
            let attribute = self.read_name();
            Lexeme::with_name(
              Location::new(start, self.position()),
              Type::ATTRIBUTE,
              attribute.0,
            )
          } else {
            // 空属性名（cpp `""` 字面量）：静态 NUL 结尾空字节串，非 null 空名
            Lexeme::with_name(
              Location::new(start, self.position()),
              Type::ATTRIBUTE,
              EMPTY_ATTRIBUTE_NAME,
            )
          }
        }
      }

      _ => {
        if is_digit(self.peekch()) {
          self.read_number(&start, self.offset)
        } else if is_identifier_start_char(self.peekch()) {
          let name = self.read_name();
          Lexeme::with_name(Location::new(start, self.position()), name.1, name.0)
        } else if ((self.peekch() as u8) & 0x80) != 0 {
          self.read_utf_8_error()
        } else {
          let ch = self.peekch();
          self.consume();

          Lexeme::from_char(Location::with_length(start, 1), ch)
        }
      }
    }
  }
}
