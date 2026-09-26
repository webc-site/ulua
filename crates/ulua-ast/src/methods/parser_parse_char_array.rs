use ulua_common::{functions::c_slice::c_slice, macros::luau_assert::LUAU_ASSERT};

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

    // Safety: 本函数仅对 QUOTED_STRING/RAW_STRING/INTERP_STRING_SIMPLE 词素调用（上方 assert 记录该不变量），
    // 词法器对这些类型把字节指针写入 `Lexeme::data` 联合体的 `data` 成员，故 `data.data` 读的是活跃成员（对齐 cpp 联合体直读）。
    let data_ptr = unsafe { current_lexeme.data.data };
    let length = current_lexeme.get_length() as usize;
    // Safety: STRING/RAW/INTERP 词素的 data/length 由词法器成对写入，指向源缓冲
    // 内合法字节；空字面量退化为空切片。
    let bytes = unsafe { c_slice(data_ptr, length) };

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
