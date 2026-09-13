use crate::{
  enums::config_status::ConfigStatus, functions::convert_config_status::convert_config_status,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn get_config_status(&self) -> ConfigStatus {
    unsafe { self.config.as_ref() }
      .and_then(|config| config.get_config_status)
      .map_or(ConfigStatus::Absent, |status| {
        convert_config_status(unsafe { status(self.l, self.ctx) })
      })
  }
}
