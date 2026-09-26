//! `AstName AstNameTable::get(const char* name) const` — Ast/src/Lexer.cpp:264.
//!
//! Rust 侧入口按借用形态收口：字节切片 [`AstNameTable::get_slice`]、
//! 字符串切片 [`AstNameTable::get_str`]。持有 C ABI 镜像串的调用点在
//! 边界处 `to_bytes()` 一次后走 `get_slice`，NUL 语义不再进入本表 API。

use crate::records::{ast_name::AstName, ast_name_table::AstNameTable};

impl AstNameTable {
  /// Looks up a name by byte slice (不含 NUL 终止符的原始字节)。
  #[inline]
  pub fn get_slice(&self, name: &[u8]) -> AstName {
    self.get_with_type(name).0
  }

  /// Looks up a name by string slice.
  #[inline]
  pub fn get_str(&self, name: &str) -> AstName {
    self.get_slice(name.as_bytes())
  }
}
