use ulua_cli_lib::functions::is_absolute_path::is_absolute_path;

use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
  type_aliases::navigate_result::NavigateResult,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_jump_to_alias(
  this: *mut FileNavigationContext,
  path: &str,
) -> NavigateResult {
  unsafe {
    let this = &mut *this;

    if !is_absolute_path(path) {
      return NavigateResult::NotFound;
    }

    let status = this.vfs.reset_to_path(path);
    convert(status)
  }
}
