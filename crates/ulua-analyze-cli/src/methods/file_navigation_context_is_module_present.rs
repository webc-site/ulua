use ulua_cli_lib::functions::is_file::is_file;

use crate::records::file_navigation_context::FileNavigationContext;

impl FileNavigationContext {
  pub fn is_module_present(&self) -> bool {
    is_file(&self.vfs.get_absolute_file_path())
  }
}
