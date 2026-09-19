use ulua_cli_lib::methods::vfs_navigator_reset_to_std_in::vfs_navigator_reset_to_std_in;
use ulua_require::enums::navigate_result::NavigateResult;

use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
};

/// `NavigateResult FileNavigationContext::resetToRequirer()` (`CLI/src/AnalyzeRequirer.cpp:37-42`):
///
/// ```cpp
/// if (requirerPath == "-")
///     return convert(vfs.resetToStdIn());
/// return convert(vfs.resetToPath(requirerPath));
/// ```
///
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn file_navigation_context_reset_to_requirer(
  this: *mut FileNavigationContext,
) -> NavigateResult {
  unsafe {
    let this = &mut *this;

    if this.requirer_path == "-" {
      return convert(vfs_navigator_reset_to_std_in(&mut this.vfs));
    }

    // reset_to_path 可变借用 vfs 字段、共享借用 requirer_path 字段，字段不相交，免 clone
    convert(this.vfs.reset_to_path(&this.requirer_path))
  }
}
