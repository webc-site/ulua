use ulua_require::enums::config_behavior::ConfigBehavior;

use crate::records::file_navigation_context::FileNavigationContext;

impl FileNavigationContext {
  pub fn get_config_behavior(&self) -> ConfigBehavior {
    ConfigBehavior::GetConfig
  }
}
