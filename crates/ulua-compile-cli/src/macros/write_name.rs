/// 输出 `"key": ` 前缀 (cpp `#define WRITE_NAME`)
macro_rules! WRITE_NAME {
  ($out:expr, $indent:expr, $key:literal) => {
    write!($out, concat!($indent, "\"", $key, "\": "))?
  };
}

pub(crate) use WRITE_NAME;
