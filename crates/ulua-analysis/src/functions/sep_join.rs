//! 分隔符拼接的单点工具。
//!
//! C++ 侧 `if (!first) buf += sep;` 型首元素旗标循环在 `Error.cpp` 的多个
//! `operator()` 里逐字重复；Rust 直译把它们抄成了 8 处手写 `let mut first`
//! 循环。收口为一个可复用写入器：逐元素写入，非首元素前自动补分隔符，
//! 语义与原循环逐字节一致（分隔符只在两个元素之间出现，不前缀不后缀）。

use alloc::string::String;

/// 向 `buf` 依序写入元素并在非首元素前补 `sep` 的写入器。
pub(crate) struct SepWriter<'a> {
  buf: &'a mut String,
  sep: &'a str,
  first: bool,
}

impl<'a> SepWriter<'a> {
  pub fn new(buf: &'a mut String, sep: &'a str) -> Self {
    SepWriter {
      buf,
      sep,
      first: true,
    }
  }

  /// 写入一个元素：等价 C++ `if (!first) { buf += sep; } first = false; buf += item;`。
  pub fn push(&mut self, item: &str) {
    if !self.first {
      self.buf.push_str(self.sep);
    }
    self.first = false;
    self.buf.push_str(item);
  }
}
