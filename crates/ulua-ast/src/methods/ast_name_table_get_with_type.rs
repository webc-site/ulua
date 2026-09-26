//! `std::pair<AstName, Lexeme::Type> AstNameTable::get_with_type(const char* name, size_t length) const`
//! — Ast/src/Lexer.cpp:245.

use crate::{
  enums::type_lexer::Type,
  records::{ast_name::AstName, ast_name_table::AstNameTable, entry::Entry},
};

impl AstNameTable {
  /// 字节切片查表内核（cpp `(const char* name, size_t length)` 成对入参折为单一
  /// 切片，指针与长度不再可漂移），仅供本 crate 读法路径（`get_slice`、
  /// `Lexer::read_name`）使用；对外一律走切片/字符串门面。
  ///
  /// 判定逐位 ⇔ cpp：键 `Entry.value` 存入参地址值（指针 identity 键即 AstName
  /// 本身），命中与否仍由 hash/eq 的内容判定（`records::entry.rs`：FNV 与
  /// 长度+字节比较，均只读前 `length` 字节）决定；未命中返回 null 名 + NAME
  /// （cpp `return std::make_pair(AstName(), Lexeme::Name)`）。
  pub(crate) fn get_with_type(&self, name: &[u8]) -> (AstName, Type) {
    let key = Entry {
      // 位拷贝契约：只透传地址值，不构造引用（空切片的 dangling 指针与 cpp
      // 传 `buf+len` 一指同样永不被读）。
      value: AstName {
        value: name.as_ptr(),
        len: name.len() as u32,
      },
      length: name.len() as u32,
      r#type: Type::EOF,
    };

    if let Some(entry) = self.data.find(&key) {
      (entry.value, entry.r#type)
    } else {
      (AstName::new(), Type::NAME)
    }
  }
}
