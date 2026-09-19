use crate::{
  enums::navigate_result::NavigateResult,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext<'_> {
  pub fn to_alias_fallback(&mut self, alias_unprefixed: &[u8]) -> NavigateResult {
    let config = unsafe { self.config.as_ref() };
    self.call_with_c_str(
      config.and_then(|config| config.to_alias_fallback),
      alias_unprefixed,
    )
  }
}
