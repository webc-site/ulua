use crate::functions::bad_setting::{OPT_BOOL, SETTING_FALSE, SETTING_TRUE, bad_setting};

/// cpp: `static Error parseBoolean(bool& result, const std::string& value)`。
/// 结果完全由 `value` 决定，故取消 C++ 的 `bool&` 出参，改为返回
/// `Result<bool, String>`（`Err` 对应 cpp 的 `Error`）；解析失败时调用方
/// 保持字段原值，与 cpp 语义一致。
pub(crate) fn parse_boolean(value: &str) -> Result<bool, String> {
  match value {
    SETTING_TRUE => Ok(true),
    SETTING_FALSE => Ok(false),
    _ => Err(bad_setting(value, OPT_BOOL)),
  }
}
