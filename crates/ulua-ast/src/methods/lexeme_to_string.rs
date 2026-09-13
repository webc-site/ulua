//! `Lexeme::to_string` (`Ast/src/Lexer.cpp`).
//!
//! `Lexeme::Type` is a newtype over `i32`, so its named values are associated
//! consts used here as constant match patterns (`Type::EOF`), not glob-imported
//! enum variants. `Lexeme::data` is a C `union`, so the payload arms read the
//! active member through `unsafe { self.data.<member> }` rather than matching it.

use core::{
  fmt::{Display, Formatter, Result},
  slice::from_raw_parts,
  str::from_utf8_unchecked,
};

use crate::{
  functions::find_confusable::find_confusable,
  records::lexeme::{Lexeme, Type},
};

const K_RESERVED: [&str; 21] = [
  "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local",
  "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
];

impl Display for Lexeme {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let ptr = unsafe { self.data.data };
    match self.r#type {
      Type::EOF => write!(f, "<eof>"),
      Type::EQUAL => write!(f, "'=='"),
      Type::LESS_EQUAL => write!(f, "'<='"),
      Type::GREATER_EQUAL => write!(f, "'>='"),
      Type::NOT_EQUAL => write!(f, "'~='"),
      Type::DOT2 => write!(f, "'..'"),
      Type::DOT3 => write!(f, "'...'"),
      Type::SKINNY_ARROW => write!(f, "'->'"),
      Type::DOUBLE_COLON => write!(f, "'::'"),
      Type::FLOOR_DIV => write!(f, "'//'"),
      Type::ADD_ASSIGN => write!(f, "'+='"),
      Type::SUB_ASSIGN => write!(f, "'-='"),
      Type::MUL_ASSIGN => write!(f, "'*='"),
      Type::DIV_ASSIGN => write!(f, "'/='"),
      Type::FLOOR_DIV_ASSIGN => write!(f, "'//='"),
      Type::MOD_ASSIGN => write!(f, "'%='"),
      Type::POW_ASSIGN => write!(f, "'^='"),
      Type::CONCAT_ASSIGN => write!(f, "'..='"),

      Type::RAW_STRING | Type::QUOTED_STRING => {
        if !ptr.is_null() {
          let s =
            unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, self.length as usize)) };
          write!(f, "\"{}\"", s)
        } else {
          write!(f, "string")
        }
      }
      Type::INTERP_STRING_BEGIN => {
        if !ptr.is_null() {
          let s =
            unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, self.length as usize)) };
          write!(f, "`{}{{", s)
        } else {
          write!(f, "the beginning of an interpolated string")
        }
      }
      Type::INTERP_STRING_MID => {
        if !ptr.is_null() {
          let s =
            unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, self.length as usize)) };
          write!(f, "}}{}{{", s)
        } else {
          write!(f, "the middle of an interpolated string")
        }
      }
      Type::INTERP_STRING_END => {
        if !ptr.is_null() {
          let s =
            unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, self.length as usize)) };
          write!(f, "}}{}`", s)
        } else {
          write!(f, "the end of an interpolated string")
        }
      }
      Type::INTERP_STRING_SIMPLE => {
        if !ptr.is_null() {
          let s =
            unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, self.length as usize)) };
          write!(f, "`{}`", s)
        } else {
          write!(f, "interpolated string")
        }
      }
      Type::NUMBER => {
        if !ptr.is_null() {
          let s =
            unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, self.length as usize)) };
          write!(f, "'{}'", s)
        } else {
          write!(f, "number")
        }
      }

      Type::NAME => {
        let name = self.name();
        if !name.is_null() {
          write!(f, "'{}'", name)
        } else {
          write!(f, "identifier")
        }
      }
      Type::COMMENT => write!(f, "comment"),
      Type::ATTRIBUTE => {
        let name = self.name();
        if !name.is_null() {
          write!(f, "'{}'", name)
        } else {
          write!(f, "attribute")
        }
      }

      Type::ATTRIBUTE_OPEN => write!(f, "'@['"),
      Type::BROKEN_STRING => write!(f, "malformed string"),
      Type::BROKEN_COMMENT => write!(f, "unfinished comment"),
      Type::BROKEN_INTERP_DOUBLE_BRACE => {
        write!(f, "'{{', which is invalid (did you mean '\\{{'?)")
      }

      Type::BROKEN_UNICODE => {
        let cp = unsafe { self.data.codepoint };
        if cp != 0 {
          if let Some(confusable) = find_confusable(cp) {
            write!(
              f,
              "Unicode character U+{:x} (did you mean '{}'?)",
              cp, confusable
            )
          } else {
            write!(f, "Unicode character U+{:x}", cp)
          }
        } else {
          write!(f, "invalid UTF-8 sequence")
        }
      }

      _ => {
        let type_val = self.r#type.0;
        if type_val < Type::CHAR_END.0 {
          write!(f, "'{}'", type_val as u8 as char)
        } else if (Type::RESERVED_BEGIN.0..Type::RESERVED_END_TOKEN.0).contains(&type_val) {
          let index = (type_val - Type::RESERVED_BEGIN.0) as usize;
          write!(f, "'{}'", K_RESERVED[index])
        } else {
          write!(f, "<unknown>")
        }
      }
    }
  }
}
