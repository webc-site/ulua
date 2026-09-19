use alloc::{string::String, vec::Vec};
use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

impl FileNavigationContext {
  pub fn get_config(&self) -> Option<Vec<u8>> {
    // `VfsNavigator::get_config` 仍以 `String` 暴露配置内容（ulua-cli-lib 的接口），
    // 这里是唯一的转换点：`into_bytes` 不改动任何字节。
    self.vfs.get_config().map(String::into_bytes)
  }
}
