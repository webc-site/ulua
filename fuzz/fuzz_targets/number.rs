// Port of Luau's `fuzz/number.cpp`: fuzz the numeric-literal parsers (the
// schubfach-adjacent paths are bug-prone). They must never panic — only return
// a `ConstantNumberParseResult`.

use std::str::from_utf8;

#[cfg(not(feature = "afl-runtime"))]
include!("standalone.rs");

use ulua_ast::functions::{
  parse_double::parse_double, parse_integer::parse_integer, parse_integer_64::parse_integer_64,
};

fn exercise_input(data: &[u8]) {
  if let Ok(s) = from_utf8(data) {
    // 解析函数已改为元组返回 `(结果, 值)`，不再写传出参数。
    let _ = parse_double(s);
    // `parse_integer` handles binary/hex literals only (it asserts base 2 or
    // 16 — base-10 integers go through `parse_integer_64` / `parse_double`).
    let _ = parse_integer(s, 2);
    let _ = parse_integer(s, 16);
    let _ = parse_integer_64(s, 10);
    let _ = parse_integer_64(s, 16);
  }
}

ulua_fuzz::fuzz_main!(exercise_input);
