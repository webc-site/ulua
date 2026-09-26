use alloc::{string::String, vec::Vec};

use crate::records::json_emitter::JsonEmitter;

pub(crate) const CHUNK_SIZE: usize = 4096;

pub(crate) fn append_chunk(chunks: &mut Vec<String>, sv: &str) {
  if sv.len() > CHUNK_SIZE {
    chunks.push(sv.to_owned());
    chunks.push(String::with_capacity(CHUNK_SIZE));
    return;
  }

  if chunks.is_empty() {
    chunks.push(String::with_capacity(CHUNK_SIZE));
  }

  // 上方 is_empty 分支已补 push，chunks 恒非空。
  let chunk = chunks
    .last_mut()
    .expect("上方 is_empty 即 push 蕴含 chunks 非空");
  let available = CHUNK_SIZE.saturating_sub(chunk.len());
  if sv.len() < available {
    chunk.push_str(sv);
    return;
  }

  let prefix = sv.floor_char_boundary(available);
  let (head, tail) = sv.split_at(prefix);
  chunk.push_str(head);
  let mut next = String::with_capacity(CHUNK_SIZE);
  next.push_str(tail);
  chunks.push(next);
}

impl JsonEmitter {
  pub fn write_raw_string_view(&mut self, sv: &str) {
    append_chunk(&mut self.chunks, sv);
  }

  // writeRaw(char) — pinned overload name
  /// 单字节写入（cpp `writeRaw(char)`，AstJsonEncoder.cpp:156 形态）。旧名
  /// `write_raw_c_char`/`AsRawByte` 照抄 C 字符类型，但本类型不是 C ABI 面；
  /// 全部调用点均为 ASCII 结构性字符（`[`/`]` 等），`u8` 直收即可。
  /// 非 ASCII 字节旧实现是 `from_utf8_unchecked` 的 UB，这里按 Latin-1 码位的
  /// UTF-8 编码落盘，行为有定义；ASCII 下逐字节一致。
  #[inline]
  pub fn write_raw_byte(&mut self, c: u8) {
    let mut buf = [0u8; 4];
    let s = (c as char).encode_utf8(&mut buf);
    self.write_raw_string_view(s);
  }
}
