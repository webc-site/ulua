//! `Lexeme Lexer::read_next()` — Ast/src/Lexer.cpp:719. The single-token dispatch.

use crate::{
  enums::brace_type::BraceType,
  functions::{is_alpha::is_alpha, is_digit_lexer::is_digit},
  records::{
    lexeme::{Lexeme, Type},
    lexer::Lexer,
    location::Location,
  },
};

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

        if self.brace_stack.is_empty() {
          return Lexeme::from_char(Location::with_length(start, 1), '}');
        }

        let brace_stack_top = *self.brace_stack.last().unwrap();
        self.brace_stack.pop();

        if brace_stack_top != BraceType::InterpolatedString {
          return Lexeme::from_char(Location::with_length(start, 1), '}');
        }

        self.read_interpolated_string_section(
          start,
          Type::INTERP_STRING_MID,
          Type::INTERP_STRING_END,
        )
      }

      '=' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::EQUAL)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '=')
        }
      }

      '<' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::LESS_EQUAL)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '<')
        }
      }

      '>' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::GREATER_EQUAL)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '>')
        }
      }

      '~' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::NOT_EQUAL)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '~')
        }
      }

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

      '+' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::ADD_ASSIGN)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '+')
        }
      }

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

      '*' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::MUL_ASSIGN)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '*')
        }
      }

      '%' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::MOD_ASSIGN)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '%')
        }
      }

      '^' => {
        self.consume();
        if self.peekch() == '=' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::POW_ASSIGN)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), '^')
        }
      }

      ':' => {
        self.consume();
        if self.peekch() == ':' {
          self.consume();
          Lexeme::new(Location::with_length(start, 2), Type::DOUBLE_COLON)
        } else {
          Lexeme::from_char(Location::with_length(start, 1), ':')
        }
      }

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

          if is_alpha(self.peekch()) || self.peekch() == '_' {
            let attribute = self.read_name();
            Lexeme::with_name(
              Location::new(start, self.position()),
              Type::ATTRIBUTE,
              attribute.0.value,
            )
          } else {
            Lexeme::with_name(
              Location::new(start, self.position()),
              Type::ATTRIBUTE,
              c"".as_ptr(),
            )
          }
        }
      }

      _ => {
        if is_digit(self.peekch()) {
          self.read_number(&start, self.offset)
        } else if is_alpha(self.peekch()) || self.peekch() == '_' {
          let name = self.read_name();
          Lexeme::with_name(Location::new(start, self.position()), name.1, name.0.value)
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
