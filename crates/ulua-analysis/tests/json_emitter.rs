//! `JsonEmitter` / `AstJsonEncoder` 的字符串写出与分块（chunk）语义，原型为
//! cpp `tests/JsonEmitter.test.cpp` 与 `tests/AstJsonEncoder.test.cpp` 的
//! `writeString` 系列。
//!
//! 两条移植风险：转义表必须与 cpp 的 `CONTROL_ESCAPES` 逐字节一致；分块
//! （`CHUNK_SIZE = 4096`）切在 UTF-8 多字节序列中间时，必须按字符边界切，
//! 否则拼回来的文本与输入不等（cpp 用 `string_view` 无此约束，Rust `String`
//! 会直接 panic）。

use ulua_analysis::{
  functions::write_json_emitter::{write_json_emitter_string_view, write_string},
  records::{ast_json_encoder::AstJsonEncoder, json_emitter::JsonEmitter},
};

/// 与 `src` 里的 `pub(crate) CHUNK_SIZE` 同值：外部测试读不到那个常量，
/// 改分块大小要同步这里（`CHUNK_SIZE` 变了这些边界用例才有意义）。
const CHUNK: usize = 4096;

/// 同一份期望输出同时喂给 `JsonEmitter` 与 `AstJsonEncoder` 两条写入口。
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
  for length in [
    0,
    1,
    CHUNK - 4,
    CHUNK - 3,
    CHUNK - 2,
    CHUNK - 1,
    CHUNK,
    CHUNK * 2,
  ] {
    let prefix = "x".repeat(length);
    let input = format!("{prefix}é中\u{1f600}\0\n\\\"{prefix}");
    let expected = format!("\"{prefix}é中\u{1f600}\\u0000\\n\\\\\\\"{prefix}\"");
    check(&input, &expected);
  }
}

/// 直接走 `write_raw_string_view`：前一次写恰好把当前 chunk 填到只剩 1..n
/// 字节时，多字节字符必须整体挪到下一个 chunk。
#[test]
fn utf8_chunk_boundary_preserves_raw_string() {
  for text in ["é", "中", "\u{1f600}"] {
    for remaining in 1..text.len() {
      let prefix = "x".repeat(CHUNK - remaining);
      let suffix = text.repeat(8);
      let expected = prefix.clone() + &suffix;
      assert!(expected.len() > CHUNK);

      let mut emitter = JsonEmitter::default();
      emitter.write_raw_string_view(&prefix);
      emitter.write_raw_string_view(&suffix);
      assert_eq!(emitter.str(), expected);

      let mut encoder = AstJsonEncoder::ast_json_encoder_ast_json_encoder();
      encoder.write_raw_string_view(&prefix);
      encoder.write_raw_string_view(&suffix);
      assert_eq!(encoder.str(), expected);
    }
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
