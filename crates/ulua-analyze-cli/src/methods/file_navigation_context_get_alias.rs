use alloc::string::String;
use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_get_alias(
  _this: *const FileNavigationContext,
  _alias: &String,
) -> Option<String> {
  None
}
