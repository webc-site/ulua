use alloc::ffi::CString;

use crate::{
  enums::navigate_result::NavigateResult,
  functions::convert_navigate_result::convert_navigate_result,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn to_child(&mut self, component: &str) -> NavigateResult {
    let Ok(c_comp) = CString::new(component) else {
      return NavigateResult::NotFound;
    };
    unsafe { self.config.as_ref() }
      .and_then(|config| config.to_child)
      .map_or(NavigateResult::NotFound, |to_child| {
        convert_navigate_result(unsafe { to_child(self.l, self.ctx, c_comp.as_ptr()) })
      })
  }
}
