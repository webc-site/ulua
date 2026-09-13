use alloc::ffi::CString;

use crate::{
  enums::navigate_result::NavigateResult,
  functions::convert_navigate_result::convert_navigate_result,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn jump_to_alias(&mut self, path: &str) -> NavigateResult {
    let Ok(c_path) = CString::new(path) else {
      return NavigateResult::NotFound;
    };
    unsafe { self.config.as_ref() }
      .and_then(|config| config.jump_to_alias)
      .map_or(NavigateResult::NotFound, |jump| {
        convert_navigate_result(unsafe { jump(self.l, self.ctx, c_path.as_ptr()) })
      })
  }
}
