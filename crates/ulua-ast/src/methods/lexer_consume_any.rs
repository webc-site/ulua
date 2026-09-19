use crate::{functions::is_newline::is_newline, records::lexer::Lexer};

impl Lexer {
  #[inline(always)]
  pub(crate) fn consume_any(&mut self) {
    // peekch 带上界检查：EOF 时返回 '\0'（非换行），与 cpp `buffer[offset]`
    // 语义一致，但消除对裸指针的无检查解引用（offset 越界即 UB）。
    if is_newline(self.peekch()) {
      self.line += 1;
      self.line_offset = self.offset + 1;
    }

    self.offset += 1;
  }
}
