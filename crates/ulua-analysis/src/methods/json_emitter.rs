//! `json_emitter` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};

use crate::records::{
  array_emitter::ArrayEmitter, json_emitter::JsonEmitter, object_emitter::ObjectEmitter,
};

impl JsonEmitter {
  pub fn new_chunk(&mut self) {
    self.chunks.push(String::with_capacity(CHUNK_SIZE));
  }
}

impl JsonEmitter {
  pub fn pop_comma(&mut self, c: bool) {
    self.comma = c;
  }
}

impl JsonEmitter {
  pub fn push_comma(&mut self) -> bool {
    let current = self.comma;
    self.comma = false;
    current
  }
}

impl JsonEmitter {
  pub fn str(&mut self) -> String {
    self.chunks.join("")
  }
}

impl JsonEmitter {
  pub fn write_array(&mut self) -> ArrayEmitter<'_> {
    let comma = self.push_comma();
    self.write_raw_string_view("[");

    ArrayEmitter {
      emitter: self,
      comma,
      finished: false,
    }
  }
}

impl JsonEmitter {
  pub fn write_comma(&mut self) {
    if self.comma {
      self.write_raw_string_view(",");
    } else {
      self.comma = true;
    }
  }
}

impl JsonEmitter {
  pub fn write_object(&mut self) -> ObjectEmitter<'_> {
    let comma = self.push_comma();
    self.write_raw_string_view("{");

    ObjectEmitter {
      emitter: self,
      comma,
      finished: false,
    }
  }
}

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
