use alloc::{string::String, vec::Vec};
use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_get_config(
  this: *const FileNavigationContext,
) -> Option<Vec<u8>> {
  unsafe {
    let this = &*this;
    // `VfsNavigator::get_config` 仍以 `String` 暴露配置内容（ulua-cli-lib 的接口），
    // 这里是唯一的转换点：`into_bytes` 不改动任何字节。
    this.vfs.get_config().map(String::into_bytes)
  }
}
