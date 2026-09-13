use crate::enums::{config_status::ConfigStatus, luarequire_config_status::LuarequireConfigStatus};

pub(crate) fn convert_config_status(status: LuarequireConfigStatus) -> ConfigStatus {
  match status {
    LuarequireConfigStatus::ConfigPresentJson => ConfigStatus::PresentJson,
    LuarequireConfigStatus::ConfigPresentLuau => ConfigStatus::PresentLuau,
    LuarequireConfigStatus::ConfigAmbiguous => ConfigStatus::Ambiguous,
    LuarequireConfigStatus::ConfigAbsent => ConfigStatus::Absent,
  }
}
