use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
  type_aliases::navigate_result::NavigateResult,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_to_child(
  this: *mut FileNavigationContext,
  component: &str,
) -> NavigateResult {
  unsafe {
    let this = &mut *this;
    convert(this.vfs.to_child(component))
  }
}
