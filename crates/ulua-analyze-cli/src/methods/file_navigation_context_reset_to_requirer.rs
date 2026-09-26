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
impl FileNavigationContext {
  pub fn reset_to_requirer(&mut self) -> NavigateResult {
    if self.requirer_path == "-" {
      return convert(self.vfs.reset_to_std_in());
    }

    // reset_to_path 可变借用 vfs 字段、共享借用 requirer_path 字段，字段不相交，免 clone
    convert(self.vfs.reset_to_path(&self.requirer_path))
  }
}
