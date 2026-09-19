use alloc::format;

use ulua_ast::enums::mode::Mode;

/// cpp: `Error parseModeString(Mode& mode, const std::string& modeString, bool compat)`。
/// 结果完全由入参决定，故取消 `Mode&` 出参，改为返回 `Result<Mode, String>`
/// （`Err` 对应 cpp 的 `Error`）；失败时调用方保持原 mode，与 cpp 语义一致。
pub fn parse_mode_string(mode_string: &str, compat: bool) -> Result<Mode, String> {
  match mode_string {
    "nocheck" => Ok(Mode::NoCheck),
    "strict" => Ok(Mode::Strict),
    "nonstrict" => Ok(Mode::Nonstrict),
    // compat 模式兼容旧名 "noinfer"
    "noinfer" if compat => Ok(Mode::NoCheck),
    _ => Err(format!(
      "Bad mode \"{mode_string}\".  Valid options are nocheck, nonstrict, and strict"
    )),
  }
}
