use ulua_require::enums::navigate_result::NavigateResult;

use crate::{
  functions::convert_analyze_requirer::convert,
  records::file_navigation_context::FileNavigationContext,
};

impl FileNavigationContext {
  pub fn to_parent(&self) -> NavigateResult {
    let status = self.vfs.borrow_mut().to_parent();
    convert(status)
  }
}
