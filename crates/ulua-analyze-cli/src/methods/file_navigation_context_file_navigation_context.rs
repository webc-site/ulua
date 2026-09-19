use alloc::string::String;

use ulua_cli_lib::records::vfs_navigator::VfsNavigator;

use crate::records::file_navigation_context::FileNavigationContext;
impl FileNavigationContext {
  /// `FileNavigationContext::FileNavigationContext(std::string requirerPath)`
  /// (`CLI/src/AnalyzeRequirer.cpp:31-34`): `requirerPath(std::move(requirerPath))`.
  pub fn new(requirer_path: String) -> Self {
    FileNavigationContext {
      requirer_path,
      vfs: VfsNavigator::default(),
      interrupt_info: None,
    }
  }
}
