use alloc::fmt;

use ulua_common::records::variant::Variant2;

use crate::records::config_table_key::ConfigTableKey;

/// 对应 C++ `ConfigTableKey` 的字符串化：字符串键原样返回，数字键转十进制文本。
/// 原有 inherent `to_string` 改为 Display，调用点 `.to_string()` 形态不变。
impl fmt::Display for ConfigTableKey {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.0 {
      Variant2::V0(str) => f.write_str(str),
      Variant2::V1(number) => write!(f, "{number}"),
    }
  }
}
