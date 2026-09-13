use alloc::string::String;
use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_get_identifier(
  this: *const FileNavigationContext,
) -> Option<String> {
  unsafe {
    let this = &*this;
    Some(this.vfs.get_absolute_file_path())
  }
}
