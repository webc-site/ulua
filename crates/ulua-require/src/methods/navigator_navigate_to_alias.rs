use ulua_config::records::config::Config;

use crate::{
  enums::path_type::PathType,
  functions::{extract_alias::extract_alias, get_path_type::get_path_type},
  records::{
    alias_cycle_tracker::AliasCycleTracker,
    error_handler::ErrorHandler,
    navigation_context::NavigationContextTrait,
    navigator::{Error, Navigator},
  },
};

impl<C: NavigationContextTrait, E: ErrorHandler> Navigator<'_, C, E> {
  pub fn navigate_to_alias(
    &mut self,
    alias: &str,
    config: &Config,
    mut cycle_tracker: AliasCycleTracker,
  ) -> Error {
    let alias_string = alias.to_string();
    debug_assert!(config.aliases.contains(&alias_string));

    // 直接借用配置中的别名值，避免拷贝（cpp 同样使用引用）
    let value = config
      .aliases
      .find(&alias_string)
      .expect("alias must exist")
      .value
      .as_str();
    let path_type = get_path_type(value);

    if path_type == PathType::RelativeToCurrent || path_type == PathType::RelativeToParent {
      return self.navigate_through_path(value);
    }

    if path_type == PathType::Aliased {
      if let Some(error) = cycle_tracker.add(alias_string) {
        return Some(error);
      }

      let next_alias = extract_alias(value);
      let next_alias_string = next_alias.to_string();

      let (error, was_overridden) = self.to_alias_override(next_alias);
      if error.is_some() {
        return error;
      }
      if was_overridden {
        return self.navigate_through_path(value);
      }

      if config.aliases.contains(&next_alias_string) {
        if let Some(error) = self.navigate_to_alias(next_alias, config, cycle_tracker) {
          return Some(error);
        }
      } else {
        let mut parent_config = Config::default();
        if let Some(error) = self.navigate_to_and_populate_config(next_alias, &mut parent_config) {
          return Some(error);
        }

        if parent_config.aliases.contains(&next_alias_string) {
          if let Some(error) =
            self.navigate_to_alias(next_alias, &parent_config, AliasCycleTracker::new())
          {
            return Some(error);
          }
        } else if let Some(error) = self.to_alias_fallback(next_alias) {
          return Some(error);
        }
      }

      return self.navigate_through_path(value);
    }

    // 其余视为绝对路径：直接跳转
    self.jump_to_alias(value)
  }
}
