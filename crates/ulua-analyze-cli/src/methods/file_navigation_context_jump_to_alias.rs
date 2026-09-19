use alloc::string::String;

use ulua_cli_lib::functions::is_absolute_path::is_absolute_path;
use ulua_require::enums::navigate_result::NavigateResult;

use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
};

impl FileNavigationContext {
  pub fn jump_to_alias(&mut self, path: &[u8]) -> NavigateResult {
    // `is_absolute_path` / `VfsNavigator::reset_to_path` 属 ulua-cli-lib，仍以
    // `&str` 收参；合法 UTF-8 路径下该转换是恒等的，不改变任何判定。
    let path = String::from_utf8_lossy(path);

    if !is_absolute_path(&path) {
      return NavigateResult::NotFound;
    }

    let status = self.vfs.reset_to_path(&path);
    convert(status)
  }
}
