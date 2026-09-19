use crate::records::json_emitter::JsonEmitter;

const CONTROL_ESCAPES: [&str; 32] = [
  "\\u0000", "\\u0001", "\\u0002", "\\u0003", "\\u0004", "\\u0005", "\\u0006", "\\u0007", "\\b",
  "\\t", "\\n", "\\u000b", "\\f", "\\r", "\\u000e", "\\u000f", "\\u0010", "\\u0011", "\\u0012",
  "\\u0013", "\\u0014", "\\u0015", "\\u0016", "\\u0017", "\\u0018", "\\u0019", "\\u001a",
  "\\u001b", "\\u001c", "\\u001d", "\\u001e", "\\u001f",
];

pub(crate) fn write_string(sv: &str, mut write_raw: impl FnMut(&str)) {
  write_raw("\"");
  let mut start = 0;
  for (index, byte) in sv.bytes().enumerate() {
    let escaped = match byte {
      b'"' => "\\\"",
      b'\\' => "\\\\",
      0..=31 => CONTROL_ESCAPES[usize::from(byte)],
      _ => continue,
    };
    if start != index {
      write_raw(&sv[start..index]);
    }
    write_raw(escaped);
    start = index + 1;
  }
  if start != sv.len() {
    write_raw(&sv[start..]);
  }
  write_raw("\"");
}

pub fn write_json_emitter_string_view(emitter: &mut JsonEmitter, sv: &str) {
  write_string(sv, |part| emitter.write_raw_string_view(part));
}

#[cfg(test)]
mod tests {
  use alloc::{format, string::String};

  use super::{write_json_emitter_string_view, write_string};
  use crate::records::{ast_json_encoder::AstJsonEncoder, json_emitter::JsonEmitter};

  fn check(input: &str, expected: &str) {
    let mut emitter = JsonEmitter::default();
    write_json_emitter_string_view(&mut emitter, input);
    assert_eq!(emitter.str(), expected);

    let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
    encoder.write_string(input);
    assert_eq!(encoder.str(), expected);
  }

  #[test]
  fn cpp_write_string() {
    check(
      "foo,bar,baz,\n\"this should be escaped\"",
      "\"foo,bar,baz,\\n\\\"this should be escaped\\\"\"",
    );
  }

  #[test]
  fn cpp_write_string_escapes() {
    check("x\u{8}\u{c}\n\r\ty", "\"x\\b\\f\\n\\r\\ty\"");
    check("\u{1}\u{1f}", "\"\\u0001\\u001f\"");
    check("eé\u{1f600}", "\"eé\u{1f600}\"");
    check("", "\"\"");
    check("\0\\\"\u{7f}", "\"\\u0000\\\\\\\"\u{7f}\"");
  }

  #[test]
  fn all_control_bytes_match_cpp() {
    for byte in 0u8..32 {
      let input = char::from(byte).to_string();
      let escape = match byte {
        8 => "\\b".to_owned(),
        9 => "\\t".to_owned(),
        10 => "\\n".to_owned(),
        12 => "\\f".to_owned(),
        13 => "\\r".to_owned(),
        _ => format!("\\u{byte:04x}"),
      };
      check(&input, &format!("\"{escape}\""));
    }
  }

  #[test]
  fn unicode_and_escapes_cross_chunk_boundaries() {
    for length in [0, 1, 4092, 4093, 4094, 4095, 4096, 8192] {
      let prefix = "x".repeat(length);
      let input = format!("{prefix}é中\u{1f600}\0\n\\\"{prefix}");
      let expected = format!("\"{prefix}é中\u{1f600}\\u0000\\n\\\\\\\"{prefix}\"");
      check(&input, &expected);
    }
  }

  #[test]
  fn unescaped_text_is_written_as_one_slice() {
    let input = "é中".repeat(8192);
    let mut calls = 0;
    let mut output = String::new();
    write_string(&input, |part| {
      calls += 1;
      output.push_str(part);
    });
    assert_eq!(calls, 3);
    assert_eq!(output, format!("\"{input}\""));
  }
}
