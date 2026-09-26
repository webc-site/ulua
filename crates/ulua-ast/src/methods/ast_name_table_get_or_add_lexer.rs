//! `AstName AstNameTable::get_or_add(const char* name, size_t len)` — Ast/src/Lexer.cpp:254.
//! `AstName AstNameTable::get_or_add(const char* name)` — Ast/src/Lexer.cpp:259.
//!
//! Rust 侧入口按借用形态收口：字节切片 [`AstNameTable::get_or_add_slice`]、
//! 字符串切片 [`AstNameTable::get_or_add_str`]。裸 `char*`/`CStr` 版不再外露
//!（cpp 形参的 `const char*`/`size_t` 由切片证明取代），
//! 指针解引用统一收口在 `get_or_add_with_type` 一处。

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable};

impl AstNameTable {
  /// Looks up or adds a name from a byte slice into the table.
  #[inline]
  pub fn get_or_add_slice(&mut self, name: &[u8]) -> AstName {
    // Safety: `name` 是合法 `&[u8]`，`as_ptr()` 指向 `len()` 个可读字节（空切片为对齐悬垂哨兵，
    // `get_or_add_with_type` 长度 0 分支不读首字节、`copy_nonoverlapping` 零拷贝）。
    unsafe { self.get_or_add_with_type(name.as_ptr(), name.len()).0 }
  }

  /// Looks up or adds a name from a string slice into the table.
  #[inline]
  pub fn get_or_add_str(&mut self, name: &str) -> AstName {
    self.get_or_add_slice(name.as_bytes())
  }
}
