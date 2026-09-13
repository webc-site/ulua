use ulua_require::enums::config_behavior::ConfigBehavior;

use crate::records::file_navigation_context::FileNavigationContext;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_get_config_behavior(
  _this: *const FileNavigationContext,
) -> ConfigBehavior {
  ConfigBehavior::GetConfig
}
