//! `Lexeme::to_string` (`Ast/src/Lexer.cpp`).
//!
//! `Lexeme::Type` is a newtype over `i32`, so its named values are associated
//! consts used here as constant match patterns (`Type::EOF`), not glob-imported
//! enum variants. `Lexeme::data` is a C `union`, so the payload arms read the
//! active member through `unsafe { self.data.<member> }` rather than matching it.

use alloc::string::String;
use core::{
  fmt::{Display, Formatter, Result},
  slice::from_raw_parts,
};

use crate::{
  functions::find_confusable::find_confusable,
  records::lexeme::{Lexeme, Type},
};

const K_RESERVED: [&str; 21] = [
  "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local",
  "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
];

impl Lexeme {
  /// STRING/NUMBER/COMMENT 族词素的字节负载（源缓冲中的原始切片，**不保证**
  /// UTF-8：`"\xFF"` 之类的字面量按 cpp 词法器就是逐字节存的）；空指针返回
  /// `None`（cpp 同款判空）。调用方自行决定如何呈现（`from_utf8_lossy`）。
  ///
  /// # Safety
  /// `self.r#type` 必须是把指针写入 `data.data` 成员的变体（联合体直读）。
  pub(crate) unsafe fn data_bytes(&self) -> Option<&[u8]> {
    let ptr = unsafe { self.data.data };
    if ptr.is_null() {
      return None;
    }
    // SAFETY: 非空指针与 `length` 均由词法器成对写入，指向源缓冲内
    // `[ptr, ptr + length)` 的存活字节；此处只建切片，不解释编码。
    Some(unsafe { from_raw_parts(ptr as *const u8, self.length as usize) })
  }
}

impl Display for Lexeme {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

      Type::RAW_STRING | Type::QUOTED_STRING => match unsafe { self.data_bytes() } {
        Some(s) => write!(f, "\"{}\"", String::from_utf8_lossy(s)),
        None => write!(f, "string"),
      },
      Type::INTERP_STRING_BEGIN => match unsafe { self.data_bytes() } {
        Some(s) => write!(f, "`{}`{{", String::from_utf8_lossy(s)),
        None => write!(f, "the beginning of an interpolated string"),
      },
      Type::INTERP_STRING_MID => match unsafe { self.data_bytes() } {
        Some(s) => write!(f, "}}{}`{{", String::from_utf8_lossy(s)),
        None => write!(f, "the middle of an interpolated string"),
      },
      Type::INTERP_STRING_END => match unsafe { self.data_bytes() } {
        Some(s) => write!(f, "}}{}`", String::from_utf8_lossy(s)),
        None => write!(f, "the end of an interpolated string"),
      },
      Type::INTERP_STRING_SIMPLE => match unsafe { self.data_bytes() } {
        Some(s) => write!(f, "`{}`", String::from_utf8_lossy(s)),
        None => write!(f, "interpolated string"),
      },
      Type::NUMBER => match unsafe { self.data_bytes() } {
        Some(s) => write!(f, "'{}'", String::from_utf8_lossy(s)),
        None => write!(f, "number"),
      },

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
