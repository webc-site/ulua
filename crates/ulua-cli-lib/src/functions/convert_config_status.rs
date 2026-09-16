use ulua_require::enums::luarequire_config_status::LuarequireConfigStatus;

use crate::enums::config_status::ConfigStatus;

/// cpp `ReplRequirer` 回调把 `ConfigStatus` 报给 `luaopen_require` 时的枚举映射。
pub fn convert_config_status(status: ConfigStatus) -> LuarequireConfigStatus {
  match status {
    ConfigStatus::Ambiguous => LuarequireConfigStatus::ConfigAmbiguous,
    ConfigStatus::PresentJson => LuarequireConfigStatus::ConfigPresentJson,
    ConfigStatus::PresentLuau => LuarequireConfigStatus::ConfigPresentLuau,
    ConfigStatus::Absent => LuarequireConfigStatus::ConfigAbsent,
  }
}
