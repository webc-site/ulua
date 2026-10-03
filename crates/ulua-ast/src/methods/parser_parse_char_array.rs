use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  records::{ast_array::AstArray, lexer::Lexer, parser::Parser},
};

impl Parser {
  /// 解析字符串字面量词素。返回 `(value, original)`：`value` 是修复转义后的
  /// arena 拷贝，`original` 是修复前的原始字节拷贝（仅 `want_original` 为真时才
  /// 分配，否则为 [`AstArray::EMPTY`]），供 CST 记录字面量的源码拼写。
  /// 转义损坏时返回 `None`（此时 `original` 不外传）。
  pub(crate) fn parse_char_array(
    &mut self,
    want_original: bool,
  ) -> Option<(AstArray<u8>, AstArray<u8>)> {
    let current_lexeme = *self.lexer.current();
    let current_type = current_lexeme.r#type;
    LUAU_ASSERT!(
      current_type == Type::QUOTED_STRING
        || current_type == Type::RAW_STRING
        || current_type == Type::INTERP_STRING_SIMPLE
    );

    // 本函数仅对 QUOTED_STRING/RAW_STRING/INTERP_STRING_SIMPLE 词素调用（上方 assert 记录该不变量），
    // 三者均在 `Lexeme::data_bytes` 的负载变体族内：字节区间 `[ptr, ptr + length)`
    // 的判空与构造收口在该门面，null 负载折叠为空切片（与旧 `c_slice` 的
    // null/0 折叠逐位等价）。
    let bytes = current_lexeme.data_bytes().unwrap_or_default();

    let original = if want_original {
      self.copy_bytes(bytes)
    } else {
      AstArray::EMPTY
    };

    let mut data = bytes.to_vec();

    if current_type == Type::QUOTED_STRING || current_type == Type::INTERP_STRING_SIMPLE {
      if !Lexer::fixup_quoted_bytes(&mut data) {
        self.next_lexeme();
        return None;
      }
    } else {
      Lexer::fixup_multiline_bytes(&mut data);
    }

    let value = self.copy_bytes(&data);
    self.next_lexeme();
    Some((value, original))
  }
}
