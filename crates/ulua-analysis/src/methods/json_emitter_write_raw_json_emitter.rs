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

  let chunk = chunks.last_mut().unwrap();
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
}

#[cfg(test)]
mod tests {
  use crate::records::{ast_json_encoder::AstJsonEncoder, json_emitter::JsonEmitter};

  #[test]
  fn utf8_chunk_boundary_preserves_raw_string() {
    for text in ["é", "中", "\u{1f600}"] {
      for remaining in 1..text.len() {
        let prefix = "x".repeat(4096 - remaining);
        let suffix = text.repeat(8);
        let expected = prefix.clone() + &suffix;
        assert!(expected.len() > 4096);

        let mut emitter = JsonEmitter::default();
        emitter.write_raw_string_view(&prefix);
        emitter.write_raw_string_view(&suffix);
        assert_eq!(emitter.chunks.concat(), expected);
        assert_eq!(emitter.str(), expected);

        let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
        encoder.write_raw_string_view(&prefix);
        encoder.write_raw_string_view(&suffix);
        assert_eq!(encoder.chunks.concat(), expected);
        assert_eq!(encoder.str(), expected);
      }
    }
  }
}
