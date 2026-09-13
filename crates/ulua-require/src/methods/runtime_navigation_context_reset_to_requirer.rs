use crate::{
  enums::navigate_result::NavigateResult,
  functions::convert_navigate_result::convert_navigate_result,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn reset_to_requirer(&mut self) -> NavigateResult {
    unsafe { self.config.as_ref() }
      .and_then(|config| config.reset)
      .map_or(NavigateResult::NotFound, |reset| {
        convert_navigate_result(unsafe {
          reset(self.l, self.ctx, self.requirer_chunkname.as_ptr())
        })
      })
  }
}
