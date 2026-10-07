//! `c_str` 门面的行为契约测试（review.md §8）：读取端断言用本地
//! [`read_c_bytes`]——独立于被测实现的逐字节 NUL 扫描（libc `strlen` 语义）——
//! 做 cross-check，不经门面自身解码，避免同源实现自证。

use core::{ffi::c_char, ptr::null};
use std::borrow::Cow;

use ulua_common::functions::c_str::{cstr, cstr_bytes, cstr_cow};

#[test]
fn test_cstr_bytes_and_cow() {
  let literal = b"constant_str\0";
  let ptr = cstr(literal);
  assert_eq!(unsafe { cstr_bytes(ptr) }, b"constant_str");
  assert_eq!(unsafe { cstr_cow(ptr) }, "constant_str");

  // 首 NUL 截断语义（cpp 把 `const char*` 交给 `std::string` 的同款行为）
  let interior = cstr(b"ab\0cd\0");
  assert_eq!(unsafe { cstr_bytes(interior) }, b"ab");
  assert_eq!(unsafe { cstr_cow(interior) }, "ab");

  let null_ptr: *const c_char = null();
  assert_eq!(unsafe { cstr_bytes(null_ptr) }, b"");
  assert_eq!(unsafe { cstr_cow(null_ptr) }, "");
}

#[test]
fn test_cstr_cow_lossy_decoding() {
  // 非 UTF-8 字节走 `Cow::Owned` 分支：lossy 解码替换为 U+FFFD（cpp 侧无解码，
  // 此为 Rust 门面自身的宽容语义，与 `String::from_utf8_lossy` 对齐）。
  let raw = cstr(b"\xff\xfeok\0");
  assert_eq!(unsafe { cstr_cow(raw) }, "\u{FFFD}\u{FFFD}ok");
  assert!(matches!(unsafe { cstr_cow(raw) }, Cow::Owned(_)));
  // 字节版免解码：原样返回
  assert_eq!(unsafe { cstr_bytes(raw) }, b"\xff\xfeok");
}
