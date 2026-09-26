use crate::{enums::quote_style_cst::QuoteStyle, records::position::Position};

/// C++ `Writer` 虚基类的 Rust 对应。字符串参数按 cpp `std::string_view`
/// 语义取 `&[u8]` 逐字节处理：
/// - `write`/`write_multiline`/`literal`/`string`/`source_string`/`identifier`
///   的实参可能是词法器 fixup 后的字符串值（`"\xff"` 等任意字节，非 UTF-8），
///   必须走字节通道；
/// - `keyword`/`symbol` 实参恒为编译期字面量，保留 `&str`。
pub trait Writer {
  fn advance(&mut self, pos: &Position);
  fn newline(&mut self);
  fn space(&mut self);
  fn maybe_space(&mut self, new_pos: &Position, reserve: i32);
  fn write(&mut self, s: &[u8]);
  fn write_multiline(&mut self, s: &[u8]);
  fn identifier(&mut self, name: &[u8]);
  fn keyword(&mut self, s: &str);
  fn symbol(&mut self, s: &str);
  fn literal(&mut self, s: &[u8]);
  fn string(&mut self, s: &[u8]);
  fn source_string(&mut self, s: &[u8], quote_style: QuoteStyle, block_depth: u32);
}
