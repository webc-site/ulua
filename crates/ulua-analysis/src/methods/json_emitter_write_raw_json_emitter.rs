use alloc::{string::String, vec::Vec};
use core::str::from_utf8_unchecked;

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

pub trait AsRawByte {
  fn to_raw_byte(self) -> u8;
}

impl AsRawByte for u8 {
  #[inline]
  fn to_raw_byte(self) -> u8 {
    self
  }
}

impl AsRawByte for i8 {
  #[inline]
  fn to_raw_byte(self) -> u8 {
    self as u8
  }
}

impl JsonEmitter {
  pub fn write_raw_string_view(&mut self, sv: &str) {
    append_chunk(&mut self.chunks, sv);
  }

  // writeRaw(char) — pinned overload name
  #[inline]
  pub fn write_raw_c_char<B: AsRawByte>(&mut self, c: B) {
    let buf = [c.to_raw_byte()];
    // single char as a one-byte string view
    self.write_raw_string_view(unsafe { from_utf8_unchecked(&buf) });
  }
}
