//! 展示边界的字节→文本渲染：只有 Debug / 人类可读输出才做 lossy 转换，
//! 判定逻辑与跨 FFI 的传参一律保持原始字节（cpp 全程 `std::string` 字节流）。

use alloc::string::String;
use core::fmt::{Debug, Formatter, Result};

/// 字节串的 Debug 视图：非 UTF-8 字节渲染为 U+FFFD，仅用于展示。
pub(crate) struct Lossy<'a>(&'a [u8]);

impl Debug for Lossy<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "\"{}\"", String::from_utf8_lossy(self.0))
  }
}

/// 以 lossy 文本视图展示字节串（仅供 Debug 输出使用）。
pub(crate) fn lossy(bytes: &[u8]) -> Lossy<'_> {
  Lossy(bytes)
}
