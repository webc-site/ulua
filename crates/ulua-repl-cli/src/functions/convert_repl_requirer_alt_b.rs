use ulua_cli_lib::enums::config_status::ConfigStatus;
pub use ulua_require::enums::luarequire_config_status::LuarequireConfigStatus as luarequire_ConfigStatus;

pub fn convert(status: ConfigStatus) -> luarequire_ConfigStatus {
  match status {
    ConfigStatus::Ambiguous => luarequire_ConfigStatus::ConfigAmbiguous,
    ConfigStatus::PresentJson => luarequire_ConfigStatus::ConfigPresentJson,
    ConfigStatus::PresentLuau => luarequire_ConfigStatus::ConfigPresentLuau,
    ConfigStatus::Absent => luarequire_ConfigStatus::ConfigAbsent,
  }
}
