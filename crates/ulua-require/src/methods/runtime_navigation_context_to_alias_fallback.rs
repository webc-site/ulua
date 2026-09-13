use alloc::ffi::CString;

use crate::{
  enums::navigate_result::NavigateResult,
  functions::convert_navigate_result::convert_navigate_result,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn to_alias_fallback(&mut self, alias_unprefixed: &str) -> NavigateResult {
    let Ok(c_alias) = CString::new(alias_unprefixed) else {
      return NavigateResult::NotFound;
    };
    unsafe { self.config.as_ref() }
      .and_then(|config| config.to_alias_fallback)
      .map_or(NavigateResult::NotFound, |to_fallback| {
        convert_navigate_result(unsafe { to_fallback(self.l, self.ctx, c_alias.as_ptr()) })
      })
  }
}
