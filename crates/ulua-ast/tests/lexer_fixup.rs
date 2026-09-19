use ulua_ast::records::lexer::Lexer;

#[test]
fn escape_newline_variants() {
  let mut data = b"foo\\n\\r".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"foo\n\r");

  let mut data = b"foo\\\r\nbar".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"foo\nbar");
}

#[test]
fn escape_decimal_stops_after_three_digits() {
  let mut data = b"foo\\0324".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"foo 4");
}

#[test]
fn escape_hex_stops_after_two_digits() {
  let mut data = b"foo\\x204".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"foo 4");
}

#[test]
fn escape_unicode_braced() {
  let mut data = b"foo\\u{20}".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"foo ");

  let mut data = b"foo\\u{0451}".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"foo\xd1\x91");
}

#[test]
fn escape_z_skips_whitespace() {
  let mut data = b"foo\\z\n   bar".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"foobar");
}

#[test]
fn plain_bytes_pass_through() {
  let mut data = b"plain string \xff\xfe bytes".to_vec();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert_eq!(data, b"plain string \xff\xfe bytes");
}

#[test]
fn empty_input_passes() {
  let mut data = Vec::new();
  assert!(Lexer::fixup_quoted_bytes(&mut data));
  assert!(data.is_empty());
}

#[test]
fn broken_escapes_rejected() {
  for bad in [
    &b"\\x"[..],
    b"\\xF",
    b"\\xFO",
    b"\\u{",
    b"\\u{FO}",
    b"\\u{123456789}",
    b"\\359",
  ] {
    let mut data = bad.to_vec();
    assert!(!Lexer::fixup_quoted_bytes(&mut data), "{:?} 应非法", bad);
  }
}

#[test]
fn escape_matrix_matches_cpp() {
  let cases: &[(&[u8], &[u8])] = &[
    (b"\\xAB", b"\xAB"),
    (b"\\u{2024}", b"\xE2\x80\xA4"),
    (b"\\121", b"\x79"),
    (b"\\1x", b"\x01x"),
    (b"\\t", b"\t"),
    (b"\\n", b"\n"),
    (b"\\a", b"\x07"),
    (b"\\b", b"\x08"),
    (b"\\f", b"\x0c"),
    (b"\\r", b"\r"),
    (b"\\v", b"\x0b"),
    (b"\\\\", b"\\"),
    (b"\\'", b"'"),
    (b"\\\"", b"\""),
  ];
  for &(input, expected) in cases {
    let mut data = input.to_vec();
    assert!(Lexer::fixup_quoted_bytes(&mut data), "{:?} 应合法", input);
    assert_eq!(data, expected, "{:?}", input);
  }
}
