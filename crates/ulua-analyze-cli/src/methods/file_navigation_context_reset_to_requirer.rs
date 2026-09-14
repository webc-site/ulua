use ulua_cli_lib::methods::vfs_navigator_reset_to_std_in::vfs_navigator_reset_to_std_in;

use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
  type_aliases::navigate_result::NavigateResult,
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

    let requirer_path = this.requirer_path.clone();
    convert(this.vfs.reset_to_path(&requirer_path))
  }
}
