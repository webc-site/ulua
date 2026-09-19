use alloc::string::String;

use ulua_require::enums::navigate_result::NavigateResult;

use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_to_child(
  this: *mut FileNavigationContext,
  component: &[u8],
) -> NavigateResult {
  unsafe {
    let this = &mut *this;
    // `VfsNavigator::to_child` 仍以 `&str` 收参（ulua-cli-lib 的接口），合法
    // UTF-8 时该转换是恒等的；导航判定本身在 ulua-require 侧已按字节完成。
    let component = String::from_utf8_lossy(component);
    convert(this.vfs.to_child(&component))
  }
}
