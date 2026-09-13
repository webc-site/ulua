use crate::{
  enums::navigate_result::NavigateResult,
  functions::convert_navigate_result::convert_navigate_result,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn to_parent(&mut self) -> NavigateResult {
    unsafe { self.config.as_ref() }
      .and_then(|config| config.to_parent)
      .map_or(NavigateResult::NotFound, |to_parent| {
        convert_navigate_result(unsafe { to_parent(self.l, self.ctx) })
      })
  }
}
