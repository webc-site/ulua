use ulua_require::enums::navigate_result::NavigateResult;

use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_to_parent(
  this: *mut FileNavigationContext,
) -> NavigateResult {
  unsafe {
    let this = &mut *this;
    let status = this.vfs.to_parent();
    convert(status)
  }
}
