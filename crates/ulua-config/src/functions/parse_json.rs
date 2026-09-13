use alloc::{
  string::{String, ToString},
  vec::Vec,
};
use core::{ffi::c_char, slice::from_raw_parts};

use ulua_ast::records::{
  allocator::Allocator,
  ast_name_table::AstNameTable,
  lexeme::{Lexeme, Type},
  lexer::Lexer,
  position::Position,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{fail::fail, next::next},
  type_aliases::error::Error,
};

// 单字符 token 类型（ulua-ast 未定义的括号类，本地常量化）
const LBRACE: Type = Type(b'{' as i32);
const RBRACE: Type = Type(b'}' as i32);
const LBRACKET: Type = Type(b'[' as i32);
const RBRACKET: Type = Type(b']' as i32);
const COMMA: Type = Type(b',' as i32);

/// 取引号字符串负载（对应 C++ `std::string(lexer.current().data, getLength())`）。
fn quoted_string(lex: &Lexeme) -> String {
  // Safety: `data.data` 指向长度为 `get_length()` 的合法字符串负载
  unsafe {
    let p = lex.data.data as *const u8;
    String::from_utf8_lossy(from_raw_parts(p, lex.get_length() as usize)).into_owned()
  }
}

pub(crate) fn parse_json<Action>(contents: &str, mut action: Action) -> Error
where
  Action: FnMut(&[String], String) -> Error,
{
  let mut allocator = Allocator::new();
  let mut names = AstNameTable::new(&mut allocator);
  let mut lexer = Lexer::new(
    contents.as_ptr() as *const c_char,
    contents.len(),
    &mut names,
    Position::new(0, 0),
  );
  next(&mut lexer);

  let mut keys: Vec<String> = Vec::new();
  let mut array_top = false; // 不支持嵌套数组

  if lexer.current().r#type != LBRACE {
    return fail(&lexer, "'{'");
  }
  next(&mut lexer);

  loop {
    let ty = lexer.current().r#type;
    if array_top {
      if ty == RBRACKET {
        next(&mut lexer);
        array_top = false;

        LUAU_ASSERT!(!keys.is_empty());
        keys.pop();

        match lexer.current().r#type {
          COMMA => next(&mut lexer),
          RBRACE => {}
          _ => return fail(&lexer, "',' or '}'"),
        }
      } else if ty == Type::QUOTED_STRING {
        let lex = lexer.current();
        let value = quoted_string(lex);
        next(&mut lexer);

        if let Some(err) = action(&keys, value) {
          return Some(err);
        }

        match lexer.current().r#type {
          COMMA => next(&mut lexer),
          RBRACKET => {}
          _ => return fail(&lexer, "',' or ']'"),
        }
      } else {
        return fail(&lexer, "array element or ']'");
      }
    } else if ty == RBRACE {
      next(&mut lexer);

      if keys.is_empty() {
        if lexer.current().r#type != Type::EOF {
          return fail(&lexer, "end of file");
        }
        return None;
      }
      keys.pop();

      match lexer.current().r#type {
        COMMA => next(&mut lexer),
        RBRACE => {}
        _ => return fail(&lexer, "',' or '}'"),
      }
    } else if ty == Type::QUOTED_STRING {
      let lex = lexer.current();
      let key = quoted_string(lex);
      next(&mut lexer);

      keys.push(key);

      if lexer.current().r#type != Type::COLON {
        return fail(&lexer, "':'");
      }
      next(&mut lexer);

      let value_ty = lexer.current().r#type;
      if value_ty == LBRACE || value_ty == LBRACKET {
        array_top = value_ty == LBRACKET;
        next(&mut lexer);
      } else if value_ty == Type::QUOTED_STRING
        || value_ty == Type::RESERVED_TRUE
        || value_ty == Type::RESERVED_FALSE
      {
        let lex = lexer.current();
        let value = if value_ty == Type::QUOTED_STRING {
          quoted_string(lex)
        } else if value_ty == Type::RESERVED_TRUE {
          "true".to_string()
        } else {
          "false".to_string()
        };
        next(&mut lexer);

        if let Some(err) = action(&keys, value) {
          return Some(err);
        }

        keys.pop();

        match lexer.current().r#type {
          COMMA => next(&mut lexer),
          RBRACE => {}
          _ => return fail(&lexer, "',' or '}'"),
        }
      } else {
        return fail(&lexer, "field value");
      }
    } else {
      return fail(&lexer, "field key");
    }
  }
}
