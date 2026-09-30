//! `string.format` 格式串解析的纯函数契约（对照 cpp lstrlib.cpp 的
//! `scanformat` / `getnum`：标志去重、宽度/精度两位封顶、大小占位符上限）。

use ulua_vm::functions::{getnum::parse_getnum_bytes, scanformat::scan_format_spec};

#[test]
fn scan_format_spec_valid() {
  assert_eq!(scan_format_spec(b"d"), Ok(0));
  assert_eq!(scan_format_spec(b"5d"), Ok(1));
  assert_eq!(scan_format_spec(b"12d"), Ok(2));
  assert_eq!(scan_format_spec(b".5d"), Ok(2));
  assert_eq!(scan_format_spec(b"12.34d"), Ok(5));
  assert_eq!(scan_format_spec(b"-+ #012.34d"), Ok(10));
}

#[test]
fn scan_format_spec_invalid() {
  // Repeated flags: > 5 flags
  assert_eq!(
    scan_format_spec(b"-+-+-+d"),
    Err("invalid format (repeated flags)")
  );
  // Width > 2 digits
  assert_eq!(
    scan_format_spec(b"123d"),
    Err("invalid format (width or precision too long)")
  );
  // Precision > 2 digits
  assert_eq!(
    scan_format_spec(b".123d"),
    Err("invalid format (width or precision too long)")
  );
}

#[test]
fn getnum_default_when_no_digit() {
  assert_eq!(parse_getnum_bytes(b"", 4), Ok((4, 0)));
  assert_eq!(parse_getnum_bytes(b"xyz", 4), Ok((4, 0)));
}

#[test]
fn getnum_reads_decimal_prefix() {
  assert_eq!(parse_getnum_bytes(b"4", -1), Ok((4, 1)));
  assert_eq!(parse_getnum_bytes(b"16", -1), Ok((16, 2)));
  assert_eq!(parse_getnum_bytes(b"16xyz", -1), Ok((16, 2)));
  assert_eq!(parse_getnum_bytes(b"0", -1), Ok((0, 1)));
}

/// 上游 MAXSSIZE = (INT_MAX >> 1) + 1 = 1073741824：达到上限本身合法，
/// 再大或仍有续位数字即报 "size specifier is too large"。
#[test]
fn getnum_rejects_beyond_maxssize() {
  assert!(parse_getnum_bytes(b"99999999999999", -1).is_err());
  assert!(parse_getnum_bytes(b"1073741825", -1).is_err());
  assert_eq!(parse_getnum_bytes(b"1073741824", -1), Ok((1073741824, 10)));
}
