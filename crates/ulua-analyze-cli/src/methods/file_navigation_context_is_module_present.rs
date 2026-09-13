use ulua_cli_lib::functions::is_file::is_file;

use crate::records::file_navigation_context::FileNavigationContext;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_is_module_present(
  this: *const FileNavigationContext,
) -> bool {
  unsafe {
    let this = &*this;
    is_file(&this.vfs.get_absolute_file_path())
  }
}
