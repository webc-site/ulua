/// 输出 `"key": value` 对, 键名用上游 JSON 驼峰, 字段取 Rust snake_case (cpp `#define WRITE_PAIR`)
macro_rules! WRITE_PAIR {
  ($out:expr, $stats:expr, $indent:expr, $key:literal, $field:ident, $format:literal) => {
    write!(
      $out,
      concat!($indent, "\"", $key, "\": ", $format),
      $stats.$field
    )?
  };
}

pub(crate) use WRITE_PAIR;
