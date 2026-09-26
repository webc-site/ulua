use alloc::{
  string::{String, ToString},
  vec::Vec,
};
use core::slice::from_raw_parts;

use ulua_ast::{
  enums::type_lexer::Type,
  records::{
    allocator::Allocator, ast_name_table::AstNameTable, lexeme::Lexeme, lexer::Lexer,
    position::Position,
  },
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  error::ConfigError,
  functions::bad_setting::{SETTING_FALSE, SETTING_TRUE},
};

fn next(lexer: &mut Lexer) {
  lexer.next_lexeme();

  // skip C-style comments as Lexer only understands Lua-style comments atm
  while lexer.current().r#type == Type::FLOOR_DIV {
    lexer.nextline();
  }
}

// 单字符 token 类型（ulua-ast 未定义的括号类，本地常量化）
const LBRACE: Type = Type(b'{' as i32);
const RBRACE: Type = Type(b'}' as i32);
const LBRACKET: Type = Type(b'[' as i32);
const RBRACKET: Type = Type(b']' as i32);
const COMMA: Type = Type(b',' as i32);

/// 取引号字符串负载（对应 C++ `std::string(lexer.current().data, getLength())`）。
fn quoted_string(lex: &Lexeme) -> String {
  // Safety: 仅对 QUOTED_STRING 词元调用（调用点均已按类型分支过滤）；
  // `data.data` 指向宿主 Lexer 借用的 `contents` 缓冲区中该词元的负载，
  // 长度为 `get_length()` 字节，两者在本函数帧内均保持存活
  unsafe {
    let p = lex.data.data;
    String::from_utf8_lossy(from_raw_parts(p, lex.get_length() as usize)).into_owned()
  }
}

/// `Expected <message> at line <line>, got <lexeme> instead`（对应 C++ `fail`）。
/// 词法错误串随位置变化，统一装入 [`ConfigError::Message`]。
fn fail(lexer: &Lexer, message: &str) -> ConfigError {
  let cur = lexer.current();
  ConfigError::Message(alloc::format!(
    "Expected {} at line {}, got {} instead",
    message,
    cur.location.begin.line + 1,
    cur
  ))
}

/// 条目之后的分隔符：`,` 吃掉，`closer`（`}` 或 `]`）停住，其余报错。
fn consume_sep(lexer: &mut Lexer, closer: Type, expected: &str) -> Result<(), ConfigError> {
  match lexer.current().r#type {
    COMMA => next(lexer),
    ty if ty == closer => {}
    _ => return Err(fail(lexer, expected)),
  }
  Ok(())
}

/// 唯一调用方 `parse_config` 传入回调；收 `&mut impl FnMut` 单层签名，
/// 免去泛型转发壳。
pub(crate) fn parse_json(
  contents: &str,
  action: &mut impl FnMut(&[String], String) -> Result<(), ConfigError>,
) -> Result<(), ConfigError> {
  // Box 钉堆：AstNameTable/Lexer 内部存裸指针（捕获宿主地址），
  // 宿主一旦移动即悬垂（同 ulua-compiler tests.rs string_table! 先例）。
  let mut allocator = Box::new(Allocator::new());
  let mut names = AstNameTable::new(&mut allocator);
  // `Lexer::new` 收原始字节切片（`&[u8]`，非 UTF-8 亦可）；缓冲寿命由本作用域钉住。
  let mut lexer = Lexer::new(contents.as_bytes(), &mut names, Position::new(0, 0));
  next(&mut lexer);

  let mut keys: Vec<String> = Vec::new();
  let mut array_top = false; // 不支持嵌套数组

  if lexer.current().r#type != LBRACE {
    return Err(fail(&lexer, "'{'"));
  }
  next(&mut lexer);

  loop {
    let ty = lexer.current().r#type;
    if array_top {
      match ty {
        RBRACKET => {
          next(&mut lexer);
          array_top = false;

          LUAU_ASSERT!(!keys.is_empty());
          keys.pop();

          consume_sep(&mut lexer, RBRACE, "',' or '}'")?;
        }
        Type::QUOTED_STRING => {
          let value = quoted_string(lexer.current());
          next(&mut lexer);

          action(&keys, value)?;

          consume_sep(&mut lexer, RBRACKET, "',' or ']'")?;
        }
        _ => return Err(fail(&lexer, "array element or ']'")),
      }
    } else {
      match ty {
        RBRACE => {
          next(&mut lexer);

          if keys.is_empty() {
            if lexer.current().r#type != Type::EOF {
              return Err(fail(&lexer, "end of file"));
            }
            return Ok(());
          }
          keys.pop();

          consume_sep(&mut lexer, RBRACE, "',' or '}'")?;
        }
        Type::QUOTED_STRING => {
          let key = quoted_string(lexer.current());
          next(&mut lexer);

          keys.push(key);

          if lexer.current().r#type != Type::COLON {
            return Err(fail(&lexer, "':'"));
          }
          next(&mut lexer);

          let value_ty = lexer.current().r#type;
          match value_ty {
            LBRACE | LBRACKET => {
              array_top = value_ty == LBRACKET;
              next(&mut lexer);
            }
            Type::QUOTED_STRING | Type::RESERVED_TRUE | Type::RESERVED_FALSE => {
              let value = match value_ty {
                Type::QUOTED_STRING => quoted_string(lexer.current()),
                Type::RESERVED_TRUE => SETTING_TRUE.to_string(),
                _ => SETTING_FALSE.to_string(),
              };
              next(&mut lexer);

              action(&keys, value)?;

              keys.pop();

              consume_sep(&mut lexer, RBRACE, "',' or '}'")?;
            }
            _ => return Err(fail(&lexer, "field value")),
          }
        }
        _ => return Err(fail(&lexer, "field key")),
      }
    }
  }
}
