//! `std::pair<AstName, Lexeme::Type> Lexer::read_name()` — Ast/src/Lexer.cpp:705.

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  functions::{is_alpha::is_alpha, is_digit::is_digit},
  records::{ast_name::AstName, ast_name_table::AstNameTable, lexer::Lexer},
};

impl Lexer {
  pub(crate) fn read_name(&mut self) -> (AstName, Type) {
    LUAU_ASSERT!(is_alpha(self.peekch()) || self.peekch() == '_' || self.peekch() == '@');

    let start_offset = self.offset;

    // C++ do-while 直译：无条件消费首字符（前置 assert 已保证是名字字符），
    // 再按条件继续。条件前置一次求值，取代 loop{consume; if !cond break}
    // 的后置断点写法。
    self.consume();
    while {
      let ch = self.peekch();
      is_alpha(ch) || is_digit(ch) || ch == '_'
    } {
      self.consume();
    }

    let read_names = self.read_names;
    let data = unsafe { self.buffer.add(start_offset as usize) };
    let length = (self.offset - start_offset) as usize;
    let names: &mut AstNameTable = unsafe { &mut *self.names };

    unsafe {
      if read_names {
        names.get_or_add_with_type(data, length)
      } else {
        names.get_with_type(data, length)
      }
    }
  }
}
