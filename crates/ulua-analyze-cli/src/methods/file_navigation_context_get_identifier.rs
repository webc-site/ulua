use alloc::string::String;
use core::option::Option;

use crate::records::file_navigation_context::FileNavigationContext;

impl FileNavigationContext {
  /// cpp `std::optional<std::string> getIdentifier()`：按值返回，此处同样产出
  /// owned `String`（vfs 已为 `RefCell`，无法交出跨语句借用）。
  pub fn get_identifier(&self) -> Option<String> {
    Some(self.vfs.borrow().absolute_real_path.clone())
  }
}
