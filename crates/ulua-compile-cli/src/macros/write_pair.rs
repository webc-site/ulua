/// 输出 `"key": value` 对, 键名用上游 JSON 驼峰, 字段取 Rust snake_case (cpp `#define WRITE_PAIR`)。
/// 格式参数可省略：绝大多数调用点都是「数值 + 逗号换行」，默认 `"{},\n"` 消除重复传参。
macro_rules! WRITE_PAIR {
  ($out:expr, $stats:expr, $indent:expr, $key:literal, $field:ident, $format:literal) => {
    write!(
      $out,
      concat!($indent, "\"", $key, "\": ", $format),
      $stats.$field
    )?
  };
  // 省略格式的便捷形式：委托给完整形式
  ($out:expr, $stats:expr, $indent:expr, $key:literal, $field:ident) => {
    WRITE_PAIR!($out, $stats, $indent, $key, $field, "{},\n")
  };
}

pub(crate) use WRITE_PAIR;
