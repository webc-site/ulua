use alloc::vec::Vec;
use core::{iter::repeat_n, mem::take};

use crate::{
  enums::quote_style_cst::QuoteStyle,
  functions::{
    char_classifier::{is_digit, is_identifier_char},
    escape_bytes::escape_bytes,
  },
  records::position::Position,
};

// C++ `StringWriter : Writer`. `Writer` is an abstract base (a Rust trait), so
// `StringWriter` *implements* it rather than embedding it — the `impl Writer for
// StringWriter` lives with the PrettyPrinter methods; the inherent methods below
// are the overrides.
#[derive(Debug, Clone)]
pub struct StringWriter {
  // 字节容器：fixup 后的字符串值可含任意字节（`"\xff"` 非法 UTF-8），
  // 对应 cpp `std::string ss` 的逐字节语义，不能用 String 承载。
  pub(crate) ss: Vec<u8>,
  pub(crate) pos: Position,
  pub(crate) last_char: char,
}

impl StringWriter {
  pub(crate) fn advance(&mut self, new_pos: &Position) {
    while self.pos.line < new_pos.line {
      self.newline();
    }

    if self.pos.column < new_pos.column {
      // 补齐列差：`repeat_n` 一次 extend（零额外分配；`" ".repeat` 恒堆分配，
      // cpp 的 `std::string(count, ' ')` 小串走 SSO）；写 n 个空格后 last_char 为 ' '。
      let count = (new_pos.column - self.pos.column) as usize;
      self.ss.reserve(count);
      self.ss.extend(repeat_n(b' ', count));
      self.pos.column = new_pos.column;
      self.last_char = ' ';
    }
  }

  pub(crate) fn maybe_space(&mut self, new_pos: &Position, reserve: i32) {
    if self.pos.column + (reserve as u32) < new_pos.column {
      self.space();
    }
  }

  pub(crate) fn newline(&mut self) {
    self.ss.push(b'\n');
    self.pos.column = 0;
    self.pos.line += 1;
    self.last_char = '\n';
  }

  pub(crate) fn space(&mut self) {
    self.ss.push(b' ');
    self.pos.column += 1;
    self.last_char = ' ';
  }

  pub(crate) fn write_multiline(&mut self, s: &[u8]) {
    if s.is_empty() {
      return;
    }

    self.ss.extend_from_slice(s);
    // C++ `lastChar = s[s.size() - 1]` 取末字节；O(1) 取尾替代
    // `chars().last()` 的整串 UTF-8 解码。last_char 仅喂给
    // is_identifier_char（纯 ASCII 判定），末字节非 ASCII 时二者同为 false，
    // 观测行为一致。
    self.last_char = *s.last().unwrap_or(&0) as char;

    // memchr SIMD 单趟扫描 '\n'：行数累加，search_from 停在最后一个换行之后，
    // 即 cpp 侧 `index` 的值，无需再留一个同值变量。
    let mut num_lines = 0u32;
    let mut search_from = 0usize;
    while let Some(pos) = memchr::memchr(b'\n', &s[search_from..]) {
      num_lines += 1;
      search_from += pos + 1;
    }

    self.pos.line += num_lines;
    if num_lines > 0 {
      self.pos.column = (s.len() - search_from) as u32;
    } else {
      self.pos.column += s.len() as u32;
    }
  }

  pub(crate) fn write(&mut self, s: &[u8]) {
    if s.is_empty() {
      return;
    }

    self.ss.extend_from_slice(s);
    self.pos.column += s.len() as u32;
    // 同 write_multiline：末字节 O(1)，非 ASCII 时与 ASCII 判定无关。
    self.last_char = *s.last().unwrap_or(&0) as char;
  }

  /// cpp `StringWriter::write(char c)`：单字节写入。参数取 `u8` 而非 `char`
  /// （cpp `char` 本身就是字节类型）——`char as u8` 会静默截断非 Latin-1 值。
  /// `last_char` 仍以 cpp 的 `char`（Latin-1）形态存 `char`，`u8 as char` 无损。
  pub(crate) fn write_char(&mut self, c: u8) {
    self.ss.push(c);
    self.pos.column += 1;
    self.last_char = c as char;
  }

  // C++ StringWriter::identifier 与 keyword 函数体逐字相同（见
  // PrettyPrinter.cpp）；抽公共实现消除重复。
  fn word(&mut self, s: &[u8]) {
    if s.is_empty() {
      return;
    }

    if is_identifier_char(self.last_char) {
      self.space();
    }

    self.write(s);
  }

  pub(crate) fn identifier(&mut self, s: &[u8]) {
    self.word(s);
  }

  pub(crate) fn keyword(&mut self, s: &str) {
    self.word(s.as_bytes());
  }

  pub(crate) fn symbol(&mut self, s: &str) {
    self.write(s.as_bytes());
  }

  pub(crate) fn literal(&mut self, s: &[u8]) {
    if s.is_empty() {
      return;
    } else if is_identifier_char(self.last_char) && is_digit(s[0] as char) {
      // cpp `isDigit(s[0])`：首字节 ASCII 数字判定（s 非空已保证）。
      self.space();
    }

    self.write(s);
  }

  pub(crate) fn string(&mut self, s: &[u8]) {
    let mut quote = b'\'';
    // cpp `s.find('\'')` 的字节版：只扫单引号字节，多字节序列不受影响。
    if s.contains(&b'\'') {
      quote = b'\"';
    }

    self.write_char(quote);
    self.write(&escape_bytes(s, false));
    self.write_char(quote);
  }

  pub(crate) fn source_string(&mut self, s: &[u8], quote_style: QuoteStyle, block_depth: u32) {
    if quote_style == QuoteStyle::QuotedRaw {
      // C++ 先构造 `std::string(blockDepth, '=')` 再写三次；此处直接逐
      // 字符写 '='，省一次临时堆分配（write_char 同步推进 column）。
      self.write_char(b'[');
      for _ in 0..block_depth {
        self.write_char(b'=');
      }
      self.write_char(b'[');
      self.write_multiline(s);
      self.write_char(b']');
      for _ in 0..block_depth {
        self.write_char(b'=');
      }
      self.write_char(b']');
      return;
    }

    debug_assert!(block_depth == 0);

    // cpp `char quote = '"'` 加 switch：`default` 只留 LUAU_ASSERT、
    // 静默退回双引号。这里改为对 QuoteStyle 穷举匹配，新增变体时编译器会
    // 强制补分支；QuotedRaw 已由上方分支拦截，此臂不可达。
    let quote = match quote_style {
      QuoteStyle::QuotedDouble => b'"',
      QuoteStyle::QuotedSingle => b'\'',
      QuoteStyle::QuotedInterp => b'`',
      QuoteStyle::QuotedRaw => unreachable!("QuotedRaw 由上方分支处理"),
    };

    self.write_char(quote);
    self.write_multiline(s);
    self.write_char(quote);
  }

  /// 整体接管输出缓冲。字节直出（可能非 UTF-8），String 出口由调用方决定
  /// 语义（lossy 或原样字节）。
  pub(crate) fn take_bytes(&mut self) -> Vec<u8> {
    take(&mut self.ss)
  }
}
