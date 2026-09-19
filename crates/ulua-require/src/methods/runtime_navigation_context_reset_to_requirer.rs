use crate::{
  enums::navigate_result::NavigateResult,
  functions::{c_str_prefix::with_c_str, convert_navigate_result::convert_navigate_result},
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext<'_> {
  pub fn reset_to_requirer(&mut self) -> NavigateResult {
    unsafe { self.config.as_ref() }
      .and_then(|config| config.reset)
      .map_or(NavigateResult::NotFound, |reset| {
        let requirer_chunkname = self.requirer_chunkname;
        convert_navigate_result(with_c_str(requirer_chunkname, |chunkname| unsafe {
          reset(self.l, self.ctx, chunkname)
        }))
      })
  }
}
