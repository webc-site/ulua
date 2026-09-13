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

/// Compatibility shim for the pinned skeleton name: assigns `requirerPath` onto an
/// existing context (the in-place member-init form).
///
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_file_navigation_context(
  this: &mut FileNavigationContext,
  requirer_path: String,
) {
  this.requirer_path = requirer_path;
}
