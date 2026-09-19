use crate::{
  enums::navigate_result::NavigateResult,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext<'_> {
  pub fn jump_to_alias(&mut self, path: &[u8]) -> NavigateResult {
    let config = unsafe { self.config.as_ref() };
    self.call_with_c_str(config.and_then(|config| config.jump_to_alias), path)
  }
}
