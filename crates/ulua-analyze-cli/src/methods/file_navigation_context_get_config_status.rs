use ulua_cli_lib::enums::config_status::ConfigStatus as VfsConfigStatus;
use ulua_require::enums::config_status::ConfigStatus;

use crate::records::file_navigation_context::FileNavigationContext;

/// `ConfigStatus FileNavigationContext::getConfigStatus() const`
/// (`CLI/src/AnalyzeRequirer.cpp:73-76`): `return convert(vfs.getConfigStatus());`.
///
/// `convert` is the second `static` overload in AnalyzeRequirer.cpp (lines 20-30),
/// mapping `VfsNavigator::ConfigStatus` to `Require::NavigationContext::ConfigStatus`.
impl FileNavigationContext {
  pub fn get_config_status(&self) -> ConfigStatus {
    match self.vfs.get_config_status() {
      VfsConfigStatus::Ambiguous => ConfigStatus::Ambiguous,
      VfsConfigStatus::PresentJson => ConfigStatus::PresentJson,
      VfsConfigStatus::PresentLuau => ConfigStatus::PresentLuau,
      VfsConfigStatus::Absent => ConfigStatus::Absent,
    }
  }
}
