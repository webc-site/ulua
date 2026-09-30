use ulua_require::enums::config_status::ConfigStatus as RequireConfigStatus;

use crate::enums::config_status::ConfigStatus;

/// cpp `ReplRequirer` 回调把 `ConfigStatus` 报给 require 宿主机时的枚举映射。
pub fn convert_config_status(status: ConfigStatus) -> RequireConfigStatus {
  match status {
    ConfigStatus::Ambiguous => RequireConfigStatus::Ambiguous,
    ConfigStatus::PresentJson => RequireConfigStatus::PresentJson,
    ConfigStatus::PresentLuau => RequireConfigStatus::PresentLuau,
    ConfigStatus::Absent => RequireConfigStatus::Absent,
  }
}
