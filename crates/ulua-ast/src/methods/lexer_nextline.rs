//! `void Lexer::nextline()` — Ast/src/Lexer.cpp:393.

use crate::records::lexer::Lexer;

impl Lexer {
  pub fn nextline(&mut self) {
    // 停机字符集 {NUL, '\r', '\n'}（is_newline 仅认 '\n'）与 C++ 逐字符
    // peekch/consume 循环完全一致；memchr3 用 SIMD 单趟扫到第一个停机
    // 字节，消除逐字符函数调用推进。切片上界取 buffer.len()（cpp 的
    // bufferSize）：peekch 越界返回 '\0'，两种写法都在缓冲长度处停机。
    let start = self.offset as usize;
    if start >= self.buffer.len() {
      self.next_lexeme();
      return;
    }

    match memchr::memchr3(b'\n', b'\r', 0, &self.buffer[start..]) {
      Some(pos) => self.offset += pos as u32,
      None => self.offset = self.buffer.len() as u32,
    }

    self.next_lexeme();
  }
}
