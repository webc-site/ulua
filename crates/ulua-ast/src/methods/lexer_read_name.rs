//! `std::pair<AstName, Lexeme::Type> Lexer::read_name()` — Ast/src/Lexer.cpp:705.

use ulua_common::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  functions::char_classifier::{is_identifier_char, is_identifier_start_char},
  records::{ast_name::AstName, ast_name_table::AstNameTable, lexer::Lexer},
};

impl Lexer {
  pub(crate) fn read_name(&mut self) -> (AstName, Type) {
    LUAU_ASSERT!(is_identifier_start_char(self.peekch()) || self.peekch() == '@');

    let start_offset = self.offset;

    // C++ do-while 直译：无条件消费首字符（前置 assert 已保证是名字字符），
    // 再按条件继续。条件前置一次求值，取代 loop{consume; if !cond break}
    // 的后置断点写法。
    self.consume();
    while is_identifier_char(self.peekch()) {
      self.consume();
    }

    let read_names = self.read_names;
    // ⇔ cpp `&buffer[startOffset], offset - startOffset`（Lexer.cpp:715-716）：
    // `0 <= start_offset <= self.offset <= buffer.len()`，名字区间以单切片定界，
    // 载荷与源缓冲同寿命（records/lexer.rs 契约）。
    let name = &self.buffer[start_offset as usize..self.offset as usize];
    // Safety: `self.names` 是构造 Lexer 时接线的活 `*mut AstNameTable`（非空、比 Lexer 长寿），造 `&mut`
    // 期间 Lexer 独占该表、无其它别名借用。
    let names: &mut AstNameTable = unsafe { &mut *self.names };

    if read_names {
      // Safety: `name` 指向刚消费的名字、全部 `name.len()` 字节位于源缓冲界内，
      // 满足被调 unsafe fn 的「name 指向至少 length 个可读字节」契约；names 独占
      // 借用与 buffer 只读无别名冲突。
      unsafe { names.get_or_add_with_type(name.as_ptr().cast(), name.len()) }
    } else {
      names.get_with_type(name)
    }
  }
}
