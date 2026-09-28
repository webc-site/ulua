use alloc::fmt;

use crate::records::config_table_key::ConfigTableKey;

/// 对应 C++ `ConfigTableKey::toString`：字符串键原样返回，数字键走
/// `std::to_string(double)`（`%f`，固定 6 位小数），对齐 C++ 错误消息文本。
impl fmt::Display for ConfigTableKey {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::String(str) => f.write_str(str),
      // C++ `std::to_string(double)` 即 `%f`；浮点特殊值（NaN/inf）文本差异可忽略
      Self::F64(number) => write!(f, "{number:.6}"),
    }
  }
}
