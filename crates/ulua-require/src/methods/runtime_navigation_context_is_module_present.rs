use crate::records::runtime_navigation_context::RuntimeNavigationContext;

impl RuntimeNavigationContext {
  pub fn is_module_present(&self) -> bool {
    unsafe { self.config.as_ref() }
      .and_then(|config| config.is_module_present)
      .is_some_and(|present| unsafe { present(self.l, self.ctx) })
  }
}
