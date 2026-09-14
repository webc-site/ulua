use ulua_cli_lib::enums::config_status::ConfigStatus as VfsConfigStatus;
use ulua_require::records::navigation_context::ConfigStatus;

use crate::records::file_navigation_context::FileNavigationContext;

/// `ConfigStatus FileNavigationContext::getConfigStatus() const`
/// (`CLI/src/AnalyzeRequirer.cpp:73-76`): `return convert(vfs.getConfigStatus());`.
///
/// `convert` is the second `static` overload in AnalyzeRequirer.cpp (lines 20-30),
/// mapping `VfsNavigator::ConfigStatus` to `Require::NavigationContext::ConfigStatus`.
///
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_get_config_status(
  this: *const FileNavigationContext,
) -> ConfigStatus {
  unsafe {
    let this = &*this;
    match this.vfs.get_config_status() {
      VfsConfigStatus::Ambiguous => ConfigStatus::Ambiguous,
      VfsConfigStatus::PresentJson => ConfigStatus::PresentJson,
      VfsConfigStatus::PresentLuau => ConfigStatus::PresentLuau,
      VfsConfigStatus::Absent => ConfigStatus::Absent,
    }
  }
}
