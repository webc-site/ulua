//! `char Lexer::peekch() const` — Ast/src/Lexer.cpp:436.
//! `char Lexer::peekch(unsigned int lookahead) const` — Ast/src/Lexer.cpp:442.

use crate::records::lexer::Lexer;

impl Lexer {
  #[inline(always)]
  pub(crate) fn peekch(&self) -> char {
    // cpp `(offset < bufferSize) ? buffer[offset] : 0`：越界读折叠为 '\0'。
    // `get` 把判界与取值合一，u8→char 为无损字节值提升（cpp peekch 同形）。
    self.buffer.get(self.offset as usize).copied().unwrap_or(0) as char
  }
}

impl Lexer {
  #[inline(always)]
  pub(crate) fn peekch_ahead(&self, lookahead: u32) -> char {
    // cpp 同式的无饱和加法（offset+lookahead 与 bufferSize 比较后才读界内单字节）。
    self
      .buffer
      .get((self.offset + lookahead) as usize)
      .copied()
      .unwrap_or(0) as char
  }
}
