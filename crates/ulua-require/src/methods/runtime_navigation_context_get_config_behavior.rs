use crate::{
  enums::config_behavior::ConfigBehavior,
  records::runtime_navigation_context::RuntimeNavigationContext,
};

impl RuntimeNavigationContext {
  pub fn get_config_behavior(&self) -> ConfigBehavior {
    unsafe { self.config.as_ref() }.map_or(ConfigBehavior::GetConfig, |config| {
      if config.get_alias.is_some() {
        ConfigBehavior::GetAlias
      } else {
        ConfigBehavior::GetConfig
      }
    })
  }
}
