use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  // writeRaw(std::string_view) — pinned overload name
  pub fn write_raw_string_view(&mut self, sv: &str) {
    self.append_chunk(sv);
  }

  /// 单字节写入（cpp `writeRaw(char)`）：与 `JsonEmitter::write_raw_byte` 同形态，
  /// 去除照抄 C 字符的旧名/泛型 trait。ASCII 逐字节等价；非 ASCII 旧实现为 UB，
  /// 现按 Latin-1 码位的 UTF-8 编码落盘（行为有定义）。
  #[inline]
  pub fn write_raw_byte(&mut self, c: u8) {
    let mut buf = [0u8; 4];
    let s = (c as char).encode_utf8(&mut buf);
    self.write_raw_string_view(s);
  }
}
