use ulua_common::LUAU_ASSERT;

use crate::{functions::is_newline::is_newline, records::lexer::Lexer};

impl Lexer {
  #[inline(always)]
  pub(crate) fn consume(&mut self) {
    // consume() assumes current character is known to not be a newline; use consume_any if this is not guaranteed
    // 断言经 peekch 读取（带边界检查）：EOF 时返回 '\0' 非换行，断言仍成立，
    // 避免 debug 构建下对裸指针的无检查解引用。
    LUAU_ASSERT!(!is_newline(self.peekch()));

    self.offset += 1;
  }
}
