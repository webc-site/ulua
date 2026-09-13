use alloc::ffi::CString;

use crate::{
  enums::navigate_result::NavigateResult,
  functions::convert_navigate_result::convert_navigate_result,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn to_alias_override(&mut self, alias_unprefixed: &str) -> NavigateResult {
    let Ok(c_alias) = CString::new(alias_unprefixed) else {
      return NavigateResult::NotFound;
    };
    unsafe { self.config.as_ref() }
      .and_then(|config| config.to_alias_override)
      .map_or(NavigateResult::NotFound, |to_override| {
        convert_navigate_result(unsafe { to_override(self.l, self.ctx, c_alias.as_ptr()) })
      })
  }
}
